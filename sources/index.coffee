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
  init_k: (cb) ->
    window.onkeyup = (e) =>
      res = switch
        when e.code == 'ArrowUp' and (this.kon == 0 or this.kon == 1)
          this.kon++; true
        when e.code == 'ArrowDown' and (this.kon == 2 or this.kon == 3)
          this.kon++; true
        when e.code == 'ArrowLeft' and (this.kon == 4 or this.kon == 6)
          this.kon++; true
        when e.code == 'ArrowRight' and (this.kon == 5 or this.kon == 7)
          this.kon++; true
        when e.code == 'KeyB' and this.kon == 8 then this.kon++; true
        when e.code == 'KeyQ' and this.kon == 9 then this.concb(); false
        else false
      if res
        clearTimeout this.kontime
        this.kontime = setTimeout (=> this.kon = 0), 800
      else
        clearTimeout this.kontime
        this.kon = 0
    this.concb = cb
  kon: 0
  koncb: null
  kontime: null

initLab = ->
  clck = await clock()
  app = await lab()
  app.main_js TheLab.get_id 'othy-view'
  clck.main_js TheLab.get_id 'othy-clock'

window.TheLab = TheLab
window.launchTheLab = initLab

