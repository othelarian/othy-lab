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
  #
  deg2rad = (deg) -> deg/180*Math.PI
  createArc = (rayon, bord) ->
    arc1 = document.createElementNS 'http://www.w3.org/2000/svg', 'path'
    arcp = (ang, r) -> [Math.cos(deg2rad ang) * r, Math.sin(deg2rad ang) * r]
    [fpx, fpy] = arcp (180 - bord), rayon
    [epx, epy] = arcp bord, rayon
    arc1.setAttribute 'd', "M#{fpx} #{fpy} A#{rayon} #{rayon} 0 0 0 #{epx} #{epy}"
    arcy.append arc1
  #
  #createArc 110, 22
  #createArc 125, 19
  #createArc 140, 17
  #
  #
  #
  app.main_js getId 'othy-view'

window.TheLab = TheLab
window.launchTheLab = initLab

