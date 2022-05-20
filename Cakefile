bach = require 'bach'
coffee = require 'coffeescript'
connect = require 'connect'
fs = require 'fs'
fsp = require 'fs/promises'
http = require 'http'
lr = require 'livereload'
{ extname } = require 'path'
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
    out = null
    try
      out = coffee.compile code
    catch e
      throw e
    code: out

rollHtml = (src, dest) =>
  name: 'rolling-html'
  buildStart: ->
    this.addWatchFile src
  generateBundle: ->
    this.emitFile
      type: 'asset'
      fileName: dest
      source: fs.readFileSync src

rollMd = (src, dest) =>
  mdsLst = {}
  getMtime = (entry) -> (fs.statSync "#{src}/#{entry}").mtimeMs
  name: 'rolling-md'
  buildStart: ->
    for entry in fs.readdirSync src
      if not mdsLst.hasOwnProperty entry then mdsLst[entry] = 0
    this.addWatchFile src
  generateBundle: (options, bundle) ->
    genLst = []
    remLst = []
    for key, ptime of mdsLst
      try
        ntime = getMtime key
        if ptime < ntime
          genLst.push key
          mdsLst[key] = ntime
      catch
        remLst.push key
    for entry in fs.readdirSync src
      if not mdsLst.hasOwnProperty entry
        mdsLst[entry] = getMtime entry
        genLst.push entry
    for entry in remLst
      fs.rmSync "dist/#{dest}/#{entry}"
      delete mdsLst[entry]
    for entry in genLst
      this.emitFile
        type: 'asset'
        fileName: "#{dest}/#{entry}"
        source: fs.readFileSync "#{src}/#{entry}"

rollSass = =>
  css = ''
  name: 'rolling-sass'
  transform: (_, file) ->
    style = if envRelease then 'compressed' else 'expanded'
    rendered = sass.compile file, {style}
    css = rendered.css
    'export default ""'
  generateBundle: (options, bundle) ->
    Object.keys(bundle).forEach (filename) => delete bundle[filename]
    fs.writeFileSync options.file, css
    return null

# ACTION FUNCTIONS ####################

checkEnv = (options) ->
  if options.release then envRelease = true
  dest = if envRelease then 'docs' else 'dist'

compileJs = (cb) ->
  rustCfg =
    debug: not envRelease
  inOpts =
    input: 'sources/index.coffee'
    plugins: [
      rollCoffee()
      rust rustCfg
      ]
  outOpts =
    file: "./#{dest}/index.js"
    format: 'iife'
    assetFileNames: 'wasm/[name][extname]'
    plugins: (if envRelease then [terser()] else [])
  bundle = await rollup inOpts
  await bundle.write outOpts
  stmp = new Date().toLocaleString()
  console.log "#{stmp} => coffee/wasm compilation done"
  cb null, 0

compileSass = (cb) ->
  inOpts =
    input: 'sources/style.sass'
    plugins: [
      rollSass()
      rollHtml 'sources/index.html', 'index.html'
    ]
  if not envRelease then inOpts.plugins.push rollMd 'docs/mds', 'mds'
  outOpts =
    file: "./#{dest}/style.css"
    exports: 'default'
    format: 'cjs'
  if watching
    watcher = watch {inOpts..., output: outOpts}
    watcher.on 'event', (event) =>
      if event.code == 'ERROR' then console.log event.error
      else if event.code == 'END'
        stmp = new Date().toLocaleString()
        console.log "#{stmp} => sass/html compilation done"
    cb null, 0
  else
    sass = await rollup inOpts
    await sass.write outOpts
    stmp = new Date().toLocaleString()
    console.log "#{stmp} => sass/html compilation done"
    cb null, 0

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

task 'build', 'build the app (core + sass + wasm)', (options) ->
  checkEnv options
  console.log 'building...'
  building = bach.series createDir, compileSass, compileJs
  building (e, _) ->
    if e?
      console.log 'Something go wrong'
      console.log e
    else console.log 'building => done'

task 'clean', 'rm ./dist', (options) ->
  console.log 'cleaning `dist`...'
  rimraf './dist', (e) =>
    if e? then console.log e
    else console.log '`dist` removed successfully'

task 'sass', 'compile sass, and copy html + markdown', (options) ->
  checkEnv options
  sassing = bach.series createDir, compileSass
  sassing (e, _) -> if e? then console.log e

task 'serve', 'launch a micro server and watch files', (options) ->
  checkEnv options
  if envRelease
    console.error 'Impossible to use `serve` in `release` mode!'
  else
    watching = true
    serving = bach.series compileJs,
      (bach.parallel compileSass, launchServer)
    serving (e, _) -> if e? then console.log e

task 'wasm', 'use rollup to compile wasm and coffee', (options) ->
  checkEnv options
  compileJs => 42
