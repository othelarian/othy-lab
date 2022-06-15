import lab from '../lab/Cargo.toml'
import clock from '../clock/Cargo.toml'

getId = (id) -> document.getElementById id

TheLab =
  create_div: -> document.createElement 'div'
  get_md: (query, cbo, cbe) ->
    fetch "mds/#{query}.md"
      .then(
        (r) -> if r.ok then r.text().then(cbo) else cbe()
        (_) -> cbe())
  get_menu: -> getId('othy-menu')

initLab = ->
  #
  # TODO
  #
  clck = await clock()
  #
  app = await lab()
  #
  # TODO: lancer l'horloge
  #
  # TODO: mise en place des arcs sombres (précalcul)
  #
  gt = document.querySelector 'g.front'
  gt.addEventListener 'click', ->
    document.querySelector('g.front').classList.toggle 'front-moved'
    document.querySelector('g.back').classList.toggle 'back-moved'
  #
  #
  app.main_js getId 'othy-view'
  clck.main_js getId 'othy-clock'

window.TheLab = TheLab
window.launchTheLab = initLab

