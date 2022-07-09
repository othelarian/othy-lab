bach = require 'bach'
coffee = require 'coffeescript'
connect = require 'connect'
fs = require 'fs'
fsp = require 'fs/promises'
http = require 'http'
lr = require 'livereload'
{ extname } = require 'path'
pug = require 'pug'
rimraf = require 'rimraf'
{ rollup, watch } = require 'rollup'
rust = require '@wasm-tool/rollup-plugin-rust'
sass = require 'sass'
serveStatic = require 'serve-static'
{ terser } = require 'rollup-plugin-terser'

# OPTIONS #############################

option '-r', '--release', 'set compilation mode to release'

# GLOBAL VARS #########################

dest = ''
envRelease = false
watching = false

# ROLLUP PLUGINS ######################

rollCoffee = =>
  name: 'rolling-coffee'
  transform: (code, id) ->
    if extname(id) != '.coffee' then return null
    out = coffee.compile code
    code: out

rollStatic = (variant) =>
  rendered = ''
  name: 'rolling-static'
  transform: (_, file) ->
    switch variant
      when 'pug' then rendered = pug.renderFile file
      when 'sass'
        style = if envRelease then 'compressed' else 'expanded'
        rendered = (sass.compile file, {style}).css
    'export default ""'
  generateBundle: (options, bundle) ->
    Object.keys(bundle).forEach (filename) => delete bundle[filename]
    fs.writeFileSync options.file, rendered
    return null

# COMMON FUNS #########################

traceExec = (name) ->
  stmp = new Date().toLocaleString()
  console.log "#{stmp} => #{name} compilation done"

rollExec = (inFile, outFile, name, cb) ->
  inOpts = {input: "sources/#{inFile}", plugins: [rollStatic name]}
  outOpts = {file: "#{dest}/#{outFile}", exports: 'default', format: 'cjs'}
  if watching
    watcher = watch {inOpts..., output: outOpts}
    watcher.on 'event', (event) ->
      if event.code == 'ERROR' then console.log event.error
      else if event.code == 'END' then traceExec name
    cb null, 0
  else
    toExec = await rollup inOpts
    await toExec.write outOpts
    traceExec name
    cb null, 0

# ACTION FUNS #########################

checkEnv = (options) ->
  if options.release then envRelease = true
  dest = if envRelease then 'docs' else 'dist'

copyMds = (cb) ->
  console.log 'removing "old" mds...'
  rimraf './dist/mds', (e) =>
    if e? then console.log e
    else
      console.log 'mds deleted'
      console.log 'creating mds dir...'
      fs.mkdirSync './dist/mds'
      console.log 'copying mds...'
      fs.cpSync './docs/mds', './dist/mds/', {recursive: true}
      console.log 'mds copied'
      cb null, 0

compileJs = (cb) ->
  rustCfg = {debug: not envRelease}
  inOpts =
    input: 'sources/index.coffee'
    plugins: [rollCoffee(), rust rustCfg]
  outOpts =
    file: "./#{dest}/index.js"
    format: 'iife'
    assetFileNames: 'wasm/[name][extname]'
    plugins: (if envRelease then [terser()] else [])
  bundle = await rollup inOpts
  await bundle.write outOpts
  traceExec 'coffee/wasm'
  cb null, 0

compilePug = (cb) -> rollExec 'index.pug', 'index.html', 'pug', cb

compileSass = (cb) -> rollExec 'style.sass', 'style.css', 'sass', cb

createDir = (cb) ->
  if envRelease then return cb null, 0
  try
    await fsp.mkdir "./#{dest}"
    cb null, 0
  catch e
    if e.code = 'EEXIST'
      if not envRelease
        console.warn 'Warning: \'dist\' already exists'
      cb null, 1
    else cb e, null

launchServer = ->
  console.log 'launching server...'
  app = connect()
  app.use(serveStatic './dist')
  http.createServer(app).listen 5000
  lrServer = lr.createServer()
  console.log __dirname
  lrServer.watch './dist'

# TASKS ###############################

task 'build', 'build the app (core + static + wasm)', (options) ->
  checkEnv options
  console.log 'building...'
  building = bach.series createDir, compileSass, compilePug, compileJs
  building (e, _) ->
    if e?
      console.log 'Something go wrong'
      console.log e
    else console.log 'building => done'

task 'clean', 'rm ./dist', (options) ->
  console.log 'cleaning `dist`...'
  rimraf './dist', (e) ->
    if e? then console.log e
    else console.log '`dist` removed successfully'

task 'md', 'rm and cp -r the mds in the dist dir (force dev mode)', (_) ->
  (bach.series createDir, copyMds) (e, _) -> if e? then console.log e

task 'static', 'compile sass, and copy html + markdown', (options) ->
  checkEnv options
  compileStatic = bach.parallel compileSass, compilePug
  (bach.series createDir, compileStatic) (e, _) -> if e? then console.log e

task 'serve', 'launch a micro server and watch files', (options) ->
  checkEnv options
  if envRelease
    console.error 'Impossible to use `serve` in `release` mode!'
  else
    watching = true
    serving = bach.series compileJs, copyMds,
      (bach.parallel compileSass, compilePug, launchServer)
    serving (e, _) -> if e? then console.log e

task 'wasm', 'use rollup to compile wasm and coffee', (options) ->
  checkEnv options
  compileJs -> 42
