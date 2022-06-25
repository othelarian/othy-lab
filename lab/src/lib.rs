use std::{rc::Rc, str::FromStr};
use strum_macros::{Display, EnumString};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Window};
use yew::prelude::*;

extern crate markdown;
extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, PartialEq)]
enum OthyAppState {
  Loaded,
  Loading,
  Failed
}

#[derive(Clone, Display, EnumString, PartialEq)]
enum OthyRoute {
  #[strum(serialize="accueil")]
  Accueil,
  #[strum(serialize="blog")]
  Blog,
  #[strum(serialize="devlog")]
  Devlog,
  #[strum(serialize="jdr")]
  Jdr,
  #[strum(serialize="projets")]
  Projets
}

fn active_menu(id: usize) {
  get_id(&format!("othy-menu-back-{}", id)).class_list().toggle("moved");
  get_id(&format!("othy-menu-front-{}", id)).class_list().toggle("moved");
  get_id(&format!("othy-arc-{}", id)).class_list().toggle("active");
  get_id(&format!("othy-menu-circ-{}", id)).class_list().toggle("show");
}

fn reset_menu(old_route: &OthyRoute) {
  match old_route {
    OthyRoute::Accueil => (),
    OthyRoute::Blog => active_menu(0),
    OthyRoute::Devlog => {
      //
      // TODO
      //
    }
    OthyRoute::Jdr => active_menu(2),
    OthyRoute::Projets => active_menu(1)
  }
}

enum OthyAction {
  ChangeArgs(String),
  ChangeRoute(OthyRoute, usize)
}

#[derive(Clone, PartialEq)]
struct OthyState {
  args: Option<String>,
  app_state: OthyAppState,
  route: OthyRoute
}

impl OthyState {
  fn readhash() -> Self {
    let location = get_win().location();
    let hashd = location.hash().unwrap();
    let hashd = hashd.split("/").collect::<Vec<&str>>();
    let (route, reload) = if hashd[0].len() > 0 {
      match OthyRoute::from_str(&hashd[0][1..]) {
        Ok(r) => (r, false),
        Err(_) => (OthyRoute::Accueil, true)
      }
    } else { (OthyRoute::Accueil, false) };
    if reload { location.set_hash(&route.to_string()); }
    reset_menu(&route);
    //
    //
    // TODO: parsing des autres éléments de la route
    //
    //
    Self {
      args: None,
      app_state: OthyAppState::Loaded,
      route
    }
  }
}

impl Reducible for OthyState {
  type Action = OthyAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    match action {
      OthyAction::ChangeArgs(_) => {
        //
        // TODO: modification de l'args
        //
        self
        //
      }
      OthyAction::ChangeRoute(new_route, id) => {
        if &(*self).route != &new_route {
          let mut new_state = (*self).clone();
          reset_menu(&new_state.route);
          if id < 3 { active_menu(id); }
          get_win().location().set_hash(&new_route.to_string());
          new_state.route = new_route;
          new_state.args = None;
          Rc::new(new_state)
        } else {
          //
          // TODO: cas où il existe un args et on veut retourner à la racine de la route
          //
          self
        }
      }
    }
  }
}

#[derive(Properties, PartialEq)]
struct OthyArcProps {
  route: OthyRoute
}

#[derive(Properties, PartialEq)]
struct OthyViewProps {
  content: String
}

#[function_component(OthyView)]
fn othy_md(props: &OthyViewProps) -> Html {
  let div = create_div();
  div.set_inner_html(&*props.content);
  Html::VRef(div.into())
}

#[function_component(OthyLab)]
fn othy_lab() -> Html {
  let state = use_reducer(|| OthyState::readhash());
  let init = use_state(|| false);
  if !*init {
    // access to the blog
    let state_m = state.clone();
    let blog_cb = Callback::from(move |_: Event| {
      state_m.dispatch(OthyAction::ChangeRoute(OthyRoute::Blog, 0));
    });
    let blog_cb_m = blog_cb.clone();
    let blog_cl = Closure::<dyn Fn(_)>::wrap(Box::new(move |e| blog_cb_m.emit(e)));
    get_id("othy-menu-blog").add_event_listener_with_callback("click",
      blog_cl.as_ref().unchecked_ref()).unwrap();
    blog_cl.forget();
    let blog_cb_m = blog_cb.clone();
    let blog_cl = Closure::<dyn Fn(_)>::wrap(Box::new(move |e| blog_cb_m.emit(e)));
    get_id("othy-menu-front-0").add_event_listener_with_callback("click",
      blog_cl.as_ref().unchecked_ref()).unwrap();
    blog_cl.forget();
    // access to the projects
    let state_m = state.clone();
    let project_cb = Callback::from(move |_: Event| {
      state_m.dispatch(OthyAction::ChangeRoute(OthyRoute::Projets, 1));
    });
    let project_cb_m = project_cb.clone();
    let project_cl = Closure::<dyn Fn(_)>::wrap(Box::new(move |e| project_cb_m.emit(e)));
    get_id("othy-menu-projets").add_event_listener_with_callback("click",
      project_cl.as_ref().unchecked_ref()).unwrap();
    project_cl.forget();
    let project_cb_m = project_cb.clone();
    let project_cl = Closure::<dyn Fn(_)>::wrap(Box::new(move |e| project_cb_m.emit(e)));
    get_id("othy-menu-front-1").add_event_listener_with_callback("click",
      project_cl.as_ref().unchecked_ref()).unwrap();
    project_cl.forget();
    // access to jdr
    let state_m = state.clone();
    let jdr_cb = Callback::from(move |_: Event| {
      state_m.dispatch(OthyAction::ChangeRoute(OthyRoute::Jdr, 2));
    });
    let jdr_cb_m = jdr_cb.clone();
    let jdr_cl = Closure::<dyn Fn(_)>::wrap(Box::new(move |e| jdr_cb_m.emit(e)));
    get_id("othy-menu-jdr").add_event_listener_with_callback("click",
      jdr_cl.as_ref().unchecked_ref()).unwrap();
    jdr_cl.forget();
    let jdr_cb_m = jdr_cb.clone();
    let jdr_cl = Closure::<dyn Fn(_)>::wrap(Box::new(move |e| jdr_cb_m.emit(e)));
    get_id("othy-menu-front-2").add_event_listener_with_callback("click",
      jdr_cl.as_ref().unchecked_ref()).unwrap();
    jdr_cl.forget();
    // going back to home
    let state_m = state.clone();
    let home_cb = Callback::from(move |_: Event| {
      state_m.dispatch(OthyAction::ChangeRoute(OthyRoute::Accueil, 4));
    });
    let home_cb_m = home_cb.clone();
    let home_cl = Closure::<dyn Fn(_)>::wrap(Box::new(move |e| home_cb_m.emit(e)));
    get_id("othy-menu-circ-0").add_event_listener_with_callback("click",
      home_cl.as_ref().unchecked_ref()).unwrap();
    home_cl.forget();
    let home_cb_m = home_cb.clone();
    let home_cl = Closure::<dyn Fn(_)>::wrap(Box::new(move |e| home_cb_m.emit(e)));
    get_id("othy-menu-circ-1").add_event_listener_with_callback("click",
      home_cl.as_ref().unchecked_ref()).unwrap();
    home_cl.forget();
    let home_cb_m = home_cb.clone();
    let home_cl = Closure::<dyn Fn(_)>::wrap(Box::new(move |e| home_cb_m.emit(e)));
    get_id("othy-menu-circ-2").add_event_listener_with_callback("click",
      home_cl.as_ref().unchecked_ref()).unwrap();
    home_cl.forget();
    // handling menu
    //
    // TODO: callback sur l'affichage et le masquage du menu
    //
    //
    //
    //
    // end of the init
    init.set(true);
  }
  //
  //
  // TODO: test markdown
  //
  //
  // TODO: test des liens (mod ou hash ?)
  //
  // TODO: récupération d'un markdown
  // TODO: modification des liens
  //
  //let test_md = get_md("accueil");
  //
  let cbo = {
    let state = state.clone();
    Closure::once_into_js(
      Box::new(move |v| {
        log(&format!("get_md log: {}", v));
        //
        //state.dispatch(OthyAction::TestAction);
        //
      }) as Box<dyn Fn(String)>)
  };
  let cbe = Closure::once_into_js(Box::new(|| log("plop")) as Box<dyn Fn()>);
  //
  //
  //get_md("accueil", cbo, cbe);
  //
  match state.route {
    //
    // TODO
    //
    //OthyRoute::Accueil => html! {}, // TODO: pour l'instant on est en phase de test, à réactiver
    //plus tard
    //
    //OthyRoute::Accueil => html! {<OthyView content={test_md} />},
    //OthyRoute::Accueil => html! {<OthyView content={""} />},
    OthyRoute::Accueil => html! {"blob"},
    //
    OthyRoute::Blog => html! {"Le blog n'est pas encore prêt"},
    OthyRoute::Devlog => html!{"Vous vous êtes trompé de route..."},
    OthyRoute::Jdr => html! {"Houlà ! Il y a encore du boulot par ici !"},
    OthyRoute::Projets => {
      //
      // TODO
      //
      let oops = markdown::to_html("__something went wrong...__");
      //
      html! {<OthyView content={oops} />}
      //
    }
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = TheLab)]
  fn create_div() -> Element;

  #[wasm_bindgen(js_namespace = TheLab)]
  fn get_id(id: &str) -> Element;

  #[wasm_bindgen(js_namespace = TheLab)]
  fn get_md(query: &str, cbo: JsValue, cbe: JsValue);

  #[wasm_bindgen(js_namespace = TheLab)]
  fn get_win() -> Window;

  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub fn main_js(anchor: Element) {
  yew::start_app_in_element::<OthyLab>(anchor);
}

