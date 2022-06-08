use std::rc::Rc; use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;

extern crate markdown;

#[derive(Clone, PartialEq)]
enum OthyAppState {
  Loaded,
  Loading,
  Failed
}

#[derive(Clone, PartialEq)]
enum OthyRoute {
  Accueil,
  Blog,
  Jdr,
  Projets
}

#[derive(Clone, PartialEq)]
struct OthyState {
  app_state: OthyAppState,
  route: OthyRoute
}

enum OthyAction {
  ChangeRoute(OthyRoute),
  //
  // TODO
  TestAction
  //
}

impl OthyState {
  fn readhash() -> Self {
    //
    // TODO: récupération du hash pour déterminer quelle est la route actuelle
    //
    //
    Self {
      app_state: OthyAppState::Loaded,
      route: OthyRoute::Accueil
    }
  }
}

impl Reducible for OthyState {
  type Action = OthyAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    match action {
      OthyAction::ChangeRoute(new_route) => {
        let mut new_state = (*self).clone();
        new_state.route = new_route;
        Rc::new(new_state)
      }
      //
      // TODO
      //
      OthyAction::TestAction => {
        log("action test");
        self
      }
      //
    }
  }
}

#[derive(Properties, PartialEq)]
struct OthyArcProps {
  route: OthyRoute
}

#[function_component(OthyArcMenu)]
fn othy_arc_menu(props: &OthyArcProps) -> Html {
  //
  // TODO
  //
  html! {"(arc menu)"}
  //
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
  //
  // TODO: création des cbs
  //
  create_portal(
    html! {<OthyArcMenu route={state.route.clone()} />},
    get_menu().into()
  );
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
        state.dispatch(OthyAction::TestAction);
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
    OthyRoute::Accueil => html! {<OthyView content={""} />},
    //
    OthyRoute::Blog => html! {"Le blog n'est pas encore prêt"},
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
  fn get_md(query: &str, cbo: JsValue, cbe: JsValue);

  #[wasm_bindgen(js_namespace = TheLab)]
  fn get_menu() -> Element;

  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub fn main_js(anchor: Element) {
  yew::start_app_in_element::<OthyLab>(anchor);
}

