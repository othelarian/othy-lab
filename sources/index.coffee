import lab from '../lab/Cargo.toml'
#import clock from '../clock/Cargo/toml'

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
  #clock = await clock
  #
  app = await lab()
  #
  # TODO: lancer l'horloge
  #
  # TODO: mise en place des arcs sombres (précalcul)
  #
  arcy = getId 'test-arc'
  console.log arcy
  #
  arc1 = document.createElement 'path'
  arc1.p = 'h40'
  arcy.append arc1
  #
  app.main_js getId 'othy-view'

window.TheLab = TheLab
window.launchTheLab = initLab

