import lab from '../lab/Cargo.toml'
import clock from '../clock/Cargo.toml'

TheLab =
  create_div: -> document.createElement 'div'
  get_id: (id) -> document.getElementById id
  get_md: (query, cbo, cbe) ->
    fetch "mds/#{query}.md"
      .then(
        (r) -> if r.ok then r.text().then(cbo) else cbe()
        (_) -> cbe())
  get_win: -> window

initLab = ->
  clck = await clock()
  app = await lab()
  #
  # TODO: ceci est une zone de test
  #gt = document.querySelector 'g.front'
  #gt.addEventListener 'click', ->
  #  document.querySelector('g.front').classList.toggle 'moved'
  #  document.querySelector('g.back').classList.toggle 'moved'
  #
  #
  app.main_js TheLab.get_id 'othy-view'
  clck.main_js TheLab.get_id 'othy-clock'

window.TheLab = TheLab
window.launchTheLab = initLab

