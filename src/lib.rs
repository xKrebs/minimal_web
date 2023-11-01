//! # minimal
//! `minimal` is a collection of utilities to get Element and HtmlElement
//!  more convenient and easier.

pub use utils::document;
pub use utils::window;
pub use utils::MinimalDocument;
pub use utils::MinimalElement;
pub use utils::MinimalHtml;
pub use utils::MinimalList;
pub use utils::MinimalWindow;
pub use utils::MinimalNode;
pub mod utils {
    use wasm_bindgen::prelude::*;
    use web_sys::{Location, Attr, Document, Element, HtmlElement, HtmlSlotElement, NodeList, Window, Node};
    use std::panic;

    //error
    #[wasm_bindgen]
    extern {
        #[wasm_bindgen(js_namespace = console)]
        fn error(msg: String);

        type Error;

        #[wasm_bindgen(constructor)]
        fn new() -> Error;

        #[wasm_bindgen(structural, method, getter)]
        fn stack(error: &Error) -> String;
    }
    //custom error
    pub trait OptionExt<T> {
        fn custom_expect(self, msg: String) -> T;
    }
    pub trait ResultExt<T, E> {
        fn custom_expect(self, msg: String) -> T;
    }
    
    impl<T> OptionExt<T> for Option<T> {
        fn custom_expect(self, msg: String) -> T {
            let msg_clone = msg.clone();
            match self {
                Some(t) => t,
                None => {
                    panic::set_hook(Box::new(move |_| {
                        error(msg.clone());
                    }));
                    panic!("{}", msg_clone)
                }
            }
        }
    }
    impl<T, E> ResultExt<T,E> for Result<T, E> {
        fn custom_expect(self, msg: String) -> T {
            let msg_clone = msg.clone();
            match self {
                Ok(t) => t,
                Err(_) => {
                    panic::set_hook(Box::new(move |_| {
                        error(msg.clone());
                    }));
                    panic!("{}", msg_clone)
                }
            }
        }
    }
    
    /// Create a Window.
    ///
    /// # Examples
    ///
    /// ```
    /// let window = Minimal::window();
    ///
    /// assert_eq!(<Window>, window);
    /// ```
    /// 
    
    pub fn window() -> Window {
        web_sys::window().expect("no window found")
    }

    /// Create a Document.
    ///
    /// # Examples
    ///
    /// ```
    /// let document = Minimal::document();
    ///
    /// assert_eq!(<Document>, document);
    ///

    pub fn document() -> Document {
        web_sys::window().expect("no window found").document_page()
    }

    /// Some function for a Window element.
    ///
    /// # Examples
    ///
    /// ```
    /// let window = Minimal::window();
    /// let document = window.document_page();
    ///
    /// assert_eq!(<Document>, document);
    /// ```

    //TRAIT
    pub trait MinimalWindow {
        /// Get document of window.
        fn document_page(&self) -> Document;
        /// Get document_element as Element.
        fn document_element_el(&self) -> Element;
        /// Get document_element as HtmlElement.
        fn document_element_html(&self) -> HtmlElement;
        /// Get width of window, same as inner_width()
        fn get_width(&self) -> f64;
        /// Get height of window, same as inner_height()
        fn get_height(&self) -> f64;
        /// Get name of window, same as name() function.
        fn get_name(&self) -> String;
        /// Get scroll_x of window, same as scroll_x() function.
        fn get_scroll_x(&self) -> f64;
        /// Get scroll_y of window, same as scroll_y() function.
        fn get_scroll_y(&self) -> f64;
        /// Get page_x_offset of window, same as page_x_offset() function.
        fn get_page_x_offset(&self) -> f64;
        /// Get page_y_offset of window, same as page_y_offset() function.
        fn get_page_y_offset(&self) -> f64;
        /// Get screen_x of window, same as screen_x() function.
        fn get_screen_x(&self) -> JsValue;
        /// Get screen_y of window, same as screen_y() function.
        fn get_screen_y(&self) -> JsValue;
        /// Get outer_width of window, same as outer_width() function.
        fn get_outer_width(&self) -> JsValue;
        /// Get outer_height of window, same as outer_height() function.
        fn get_outer_height(&self) -> JsValue;
    }

    /// Some function for a Document element.
    ///
    /// # Examples
    ///
    /// ```
    /// let window = Minimal::window();
    /// let document = window.document_page();
    /// let h1_el = document.query_selector_el("h1");
    /// let h1_html = document.query_selector_html("h1");
    /// assert_eq!(<Element>, h1_el);
    /// assert_eq!(<HtmlElement>, h1_html);
    /// ```

    pub trait MinimalDocument {
        /// Get element by id as Element.
        fn get_element_by_id_el(&self, value: &str) -> Element;
        /// Get element by id as HtmlElement.
        fn get_element_by_id_html(&self, value: &str) -> HtmlElement;
        /// Get a List of all elements by selector.
        fn query_selector_list(&self, value: &str) -> NodeList;
        /// Get element by selector as Element.
        fn query_selector_el(&self, value: &str) -> Element;
        /// Get element by selector as HtmlElement.
        fn query_selector_html(&self, value: &str) -> HtmlElement;
        /// Get document_element as Element.
        fn document_element_el(&self) -> Element;
        /// Get document_element as HtmlElement.
        fn document_element_html(&self) -> HtmlElement;
        /// Get url of document, same as url().
        fn get_url(&self) -> String;
        /// Get location of document, same as location().
        fn get_location(&self) -> Location;
        /// Get hash of document, same as hash().
        fn get_hash(&self) -> String;
        /// Get host of document, same as host().
        fn get_host(&self) -> String;
        /// Get hostname of document, same as hostname().
        fn get_hostname(&self) -> String;
        /// Get href of document, same as href().
        fn get_href(&self) -> String;
        /// Set body of document, same as set_body().
        fn set_new_body(&self, e: HtmlElement);
        /// Get default_view of document, same as default_view().
        fn get_default_view(&self) -> Window;
        /// Create Element as Element, same as create_element.
        fn create_el(&self, local_name: &str) -> Element;
        /// Create Element as HtmlElement, same as create_element.
        fn create_html(&self, local_name: &str) -> HtmlElement;
        /// Create Element as Element, same as create_element.
        fn create_el_ns(&self, namespace: &str, qualified_name: &str) -> Element;
        /// Create Element as HtmlElement, same as create_element.
        fn create_html_ns(&self, namespace: &str, qualified_name: &str) -> HtmlElement;
    }

    /// Some function for a Element element.
    ///
    /// # Examples
    ///
    /// ```
    /// let document = Minimal::document();
    /// let h1_el = document.query_selector_el("h1");
    /// assert_eq!(<Element>, h1_el);
    /// assert_eq!(<HtmlElement>, h1_el.to_html());
    /// ```

    pub trait MinimalElement {
        /// Convert Element to HtmlElement.
        fn to_html(&self) -> HtmlElement;
        /// Match if element has a class.
        fn has_class(&self, value: &str) -> bool;
        /// Add a class to Element.
        fn add_class(&self, value: &str);
        /// Remove a class to Element.
        fn remove_class(&self, value: &str);
        /// Toggle a class to Element.
        fn toggle_class(&self, value: &str);
        /// Get namespace_uri, same as namespace_uri().
        fn get_namespace_uri(&self) -> String;
        /// Get prefix, same as prefix().
        fn get_pref(&self) -> String;
        /// Get assigned_slot, same as assigned_slot().
        fn get_assigned_slot(&self) -> HtmlSlotElement;
        /// Get a List of all elements by selector.
        fn query_selector_list(&self, value: &str) -> NodeList;
        /// Get element by selector as Element.
        fn query_selector_el(&self, value: &str) -> Element;
        /// Get element by selector as HtmlElement.
        fn query_selector_html(&self, value: &str) -> HtmlElement;
        /// Get parent_element as Element.
        fn parent_element_el(&self) -> Element;
        /// Get parent_element as HtmlElement.
        fn parent_element_html(&self) -> HtmlElement;
        /// Get first_child as Element.
        fn first_child_el(&self) -> Element;
        /// Get first_child as HtmlElement.
        fn first_child_html(&self) -> HtmlElement;
        /// Get last_child as Element.
        fn last_child_el(&self) -> Element;
        /// Get last_child as HtmlElement.
        fn last_child_html(&self) -> HtmlElement;
        /// Get previous_sibling as Element.
        fn prev_sibling_el(&self) -> Element;
        /// Get previous_sibling as HtmlElement.
        fn prev_sibling_html(&self) -> HtmlElement;
        /// Get previous_element_sibling as Element.
        fn prev_element_sibling_el(&self) -> Element;
        /// Get next_element_sibling as Element.
        fn next_element_sibling_el(&self) -> Element;
        /// Get first_element_child as Element.
        fn first_element_child_el(&self) -> Element;
        /// Get last_element_child as Element.
        fn last_element_child_el(&self) -> Element;
        /// Get previous_element_sibling as HtmlElement.
        fn prev_element_sibling_html(&self) -> HtmlElement;
        /// Get next_element_sibling as HtmlElement.
        fn next_element_sibling_html(&self) -> HtmlElement;
        /// Get first_element_child as HtmlElement.
        fn first_element_child_html(&self) -> HtmlElement;
        /// Get last_element_child as HtmlElement.
        fn last_element_child_html(&self) -> HtmlElement;
        /// Get closest element as Element.
        fn closest_el(&self, value: &str) -> Element;
        /// Get closest element as HtmlElement.
        fn closest_html(&self, value: &str) -> HtmlElement;
        /// Get attribute of Element.
        fn get_attr(&self, value: &str) -> String;
        /// Get attribute node of Element.
        fn get_attr_node(&self, value: &str) -> Attr;
        /// Get attribute ns of Element.
        fn get_attr_ns(&self, namespace: &str, localname: &str) -> String;
        /// Insert adjacent element to Element.
        fn insert_adj_el<'a, 's>(&self, where_: &'s str, element: Element) -> Element;
        /// Toggle attribute to Element.
        fn toggle_attr(&self, value: &str) -> bool;
        /// Remove attribute to Element.
        fn remove_attr(&self, value: &str);
        /// Match Element with selector, same as matches().
        fn has_match(&self, value: &str) -> bool;
        /// Get node value of Element.
        fn get_node_value(&self) -> String;
        /// Get text content of Element.
        fn get_text_content(&self) -> String;
        /// Append a Node to Element, same as append_child().
        fn app_child(&self, node: Node) -> Node;
    }

    /// Some function for a HtmlElement element.
    ///
    /// # Examples
    ///
    /// ```
    /// let document = Minimal::document();
    /// let h1_html = document.query_selector_html("h1");
    /// if h1_html.has_class("test"){
    ///     h1_html.set_prop("display", "none");
    /// }
    /// ```

    pub trait MinimalHtml {
        /// Convert HtmlElement to Element.
        fn to_el(&self) -> Element;
        /// Get offset parent as Element.
        fn offset_parent_el(&self) -> Element;
        /// Get offset parent as HtmlElement.
        fn offset_parent_html(&self) -> HtmlElement;
        /// Set property of HtmlElement, same as set_property().
        fn set_prop<'a, 's>(&self, property: &str, value: &'s str);
        /// Get property of HtmlElement, same as get_property().
        fn get_prop(&self, property: &str) -> String;
        /// Remove property of HtmlElement, same as remove_property().
        fn remove_prop(&self, value: &str) -> String;
        /// Set css text, same as set_css_text().
        fn set_css(&self, value: &str);
        /// Get css text, same as css_text().
        fn get_css(&self) -> String;
    }

    /// Some function for a NodeList element.
    ///
    /// # Examples
    ///
    /// ```
    /// let document = Minimal::document();
    /// let h1_list = document.query_selector_list("h1");
    /// assert_eq!(<NodeList>, h1_list);
    /// let element = h1_list.get_html(5);
    /// assert_eq!(<HtmlElement>, element);
    /// ```
    ///
    pub trait MinimalList {
        /// Get Node of a List.
        fn get_node(&self, index: u32) -> Node;
        /// Get Node of a List as Element.
        fn get_el(&self, index: u32) -> Element;
        /// Get Node of a List as HtmlElement.
        fn get_html(&self, index: u32) -> HtmlElement;
        /// Add a class to all Nodes of a List.
        fn add_list_class(&self, value: &str);
        /// Remove a class to all Nodes of a List.
        fn remove_list_class(&self, value: &str);
    }

    /// Some function for a Node element.
    ///
    /// # Examples
    ///
    /// ```
    /// let document = Minimal::document();
    /// let h1 = document.query_selector_list("h1");
    /// let first_child_html = h1.get_node(0).to_html();
    /// assert_eq!(<HtmlElement>, first_child_html);
    /// let first_child_html = h1.first_child_html();
    /// assert_eq!(<HtmlElement>, first_child_html);
    /// let first_child_html = h1.get_html(0);
    /// assert_eq!(<HtmlElement>, first_child_html);
    /// ```
    ///
    pub trait MinimalNode {
        /// Convert a Node to Element.
        fn to_el(&self) -> Element;
        /// Convert a Node to HtmlElement.
        fn to_html(&self) -> HtmlElement;
    }

    //IMPL TRAIT

    impl MinimalWindow for Window {
        fn document_page(&self) -> Document {
            self.document().custom_expect("There's not a document.".to_owned())
        }
        fn document_element_el(&self) -> Element {
            self.document_page()
                .document_element()
                .custom_expect("There's not a document element.".to_owned())
        }
        fn document_element_html(&self) -> HtmlElement {
            self.document_page()
                .document_element_el()
                .dyn_into::<HtmlElement>()
                .custom_expect("It's not possible convert a Null document_element to Html.".to_owned())
        }
        fn get_width(&self) -> f64 {
            self.inner_width()
            .custom_expect("There's not a width of window.".to_owned())
            .as_f64()
            .custom_expect("It's not possible convert a Null width of window to f64.".to_owned())
        }
        fn get_height(&self) -> f64 {
            self.inner_height()
            .custom_expect("There's not a height of window.".to_owned())
            .as_f64()
            .custom_expect("It's not possible convert a Null height of window to f64.".to_owned())
        }
        fn get_name(&self) -> String {
            self.name().custom_expect("There's not a name of window.".to_owned())
        }
        fn get_scroll_x(&self) -> f64 {
            self.scroll_x().custom_expect("There's not a scroll_x of window.".to_owned())
        }
        fn get_scroll_y(&self) -> f64 {
            self.scroll_y().custom_expect("There's not a scroll_y of window.".to_owned())
        }
        fn get_page_x_offset(&self) -> f64 {
            self.page_x_offset().custom_expect("There's not a page_x_offset of window.".to_owned())
        }
        fn get_page_y_offset(&self) -> f64 {
            self.page_y_offset().custom_expect("There's not a page_y_offset of window.".to_owned())
        }
        fn get_screen_x(&self) -> JsValue {
            self.screen_x().custom_expect("There's not a screen_x of window.".to_owned())
        }
        fn get_screen_y(&self) -> JsValue {
            self.screen_y().custom_expect("There's not a screen_y of window.".to_owned())
        }
        fn get_outer_height(&self) -> JsValue {
            self.outer_height().custom_expect("There's not an outer_height of window.".to_owned())
        }
        fn get_outer_width(&self) -> JsValue {
            self.outer_width().custom_expect("There's not an outer_width of window.".to_owned())
        }
    }

    impl MinimalDocument for Document {
        fn get_element_by_id_el(&self, value: &str) -> Element {
            let element = self.get_element_by_id(value);
            element.custom_expect("No Element found with id : ".to_owned() + value.clone())
        }
        fn get_element_by_id_html(&self, value: &str) -> HtmlElement {
            self.get_element_by_id_el(value)
                .to_html()
        }
        fn query_selector_el(&self, value: &str) -> Element {
            let element = match self
                .query_selector(value)
                .custom_expect("No Element with this selector found : ".to_owned() + value)
            {
                Some(e) => Some(e),
                None => None,
            };
            element.custom_expect("It's not possible convert a Null to Element : ".to_owned() + value.clone())
        }
        fn query_selector_html(&self, value: &str) -> HtmlElement {
            let element = match self
                .query_selector(value)
                .custom_expect("No HtmlElement with this selector found: ".to_owned() + value.clone())
            {
                Some(e) => Some(
                    e.to_html(),
                ),
                None => None,
            };
            element.custom_expect("It's not possible convert a null Element to HtmlElement : ".to_owned() + value.clone())
        }
        fn query_selector_list(&self, value: &str) -> NodeList {
            self.query_selector_all(value).custom_expect("There's not an element with this selector in the list: ".to_owned() + value.clone())
        }
        fn document_element_el(&self) -> Element {
            self.document_element().custom_expect("There's not document element.".to_owned())
        }
        fn document_element_html(&self) -> HtmlElement {
            self.document_element_el()
                .to_html()
        }
        fn get_url(&self) -> String {
            self.url().custom_expect("No url found".to_owned())
        }
        fn get_location(&self) -> Location {
            self.location().custom_expect("No location found".to_owned())
        }
        fn get_hash(&self) -> String {
            self.get_location().hash().custom_expect("No hash found".to_owned())
        }
        fn get_host(&self) -> String {
            self.get_location().host().custom_expect("No host found".to_owned())
        }
        fn get_hostname(&self) -> String {
            self.get_location().hostname().custom_expect("No hostname found".to_owned())
        }
        fn get_href(&self) -> String {
            self.get_location().href().custom_expect("No href found".to_owned())
        }
        fn set_new_body(&self, e: HtmlElement) {
            self.set_body(Some(&e))
        }
        fn get_default_view(&self) -> Window {
            self.default_view().custom_expect("There's not a window".to_owned())
        }
        fn create_el(&self, local_name: &str) -> Element {
            self.create_element(local_name).custom_expect("There's been an error, can't create element".to_owned())
        }
        fn create_html(&self, local_name: &str) -> HtmlElement {
            self.create_el(local_name).to_html()
        }
        fn create_el_ns(&self, namespace: &str, qualified_name: &str) -> Element {
            self.create_element_ns(Some(namespace), qualified_name).custom_expect("There's been an error, can't create element".to_owned())
        }
        fn create_html_ns(&self, namespace: &str, qualified_name: &str) -> HtmlElement {
            self.create_element_ns(Some(namespace), qualified_name).custom_expect("There's been an error, can't create element".to_owned()).to_html()
        }
    }

    impl MinimalElement for Element {
        fn to_html(&self) -> HtmlElement {
            self.clone()
                .dyn_into::<HtmlElement>()
                .custom_expect("It's not possible convert a Null Element to Html".to_owned())
        }
        fn has_class(&self, value: &str) -> bool {
            if self.class_name().contains(&("".to_owned() + value)) {
                true
            } else {
                false
            }
        }
        fn toggle_class(&self, value: &str) {
            if self.has_class(value) {
                self.remove_class(value);
            } else {
                self.add_class(value);
            }
        }
        fn add_class(&self, value: &str) {
            if !self.has_class(value){
                if self.class_name().is_empty(){
                    self.set_class_name(&(self.class_name() + &"".to_owned() + value));
                }else{
                    self.set_class_name(&(self.class_name() + &" ".to_owned() + value));
                }
            }
        }
        fn remove_class(&self, value: &str) {
            if self.has_class(value){
                self.set_class_name(&(self.class_name().replace(value, "")));
                self.set_class_name(self.class_name().trim());
            }
        }
        fn parent_element_el(&self) -> Element {
            self.parent_element().custom_expect("No parent element found".to_owned())
        }
        fn parent_element_html(&self) -> HtmlElement {
            self.parent_element_el()
                .to_html()
        }
        fn query_selector_el(&self, value: &str) -> Element {
            self.query_selector(value).custom_expect("No element found for : ".to_owned() + value.clone()).custom_expect("there's been a problem".to_owned())
        }
        fn query_selector_html(&self, value: &str) -> HtmlElement {
            self.query_selector_el(value).to_html()
        }
        fn query_selector_list(&self, value: &str) -> NodeList {
            self.query_selector_all(value).custom_expect("No element found".to_owned())
        }
        fn first_child_el(&self) -> Element {
            self.first_child().custom_expect("No first child found".to_owned()).to_el()
        }
        fn first_child_html(&self) -> HtmlElement {
            self.first_child().custom_expect("No first child found".to_owned()).to_html()
        }
        fn last_child_el(&self) -> Element {
            self.last_child().custom_expect("No last child found".to_owned()).to_el()
        }
        fn last_child_html(&self) -> HtmlElement {
            self.last_child().custom_expect("No last child found".to_owned()).to_html()
        }
        fn prev_sibling_el(&self) -> Element {
            self.previous_sibling().custom_expect("No prev sibling found".to_owned()).to_el()
        }
        fn prev_sibling_html(&self) -> HtmlElement {
            self.previous_sibling().custom_expect("No prev element found".to_owned()).to_html()
        }
        fn closest_el(&self, value: &str) -> Element {
            self.closest(value).custom_expect("No closest element found".to_owned()).custom_expect("There's been a problem".to_owned())
        }
        fn closest_html(&self, value: &str) -> HtmlElement {
            self.closest_el(value).to_html()
        }
        fn get_namespace_uri(&self) -> String {
            self.namespace_uri().custom_expect("No namespace found".to_owned())
        }
        fn get_assigned_slot(&self) -> HtmlSlotElement {
            self.assigned_slot().custom_expect("No assigned slot found".to_owned())
        }
        fn first_element_child_el(&self) -> Element {
            self.first_element_child().custom_expect("No first child found".to_owned())
        }
        fn last_element_child_el(&self) -> Element {
            self.last_element_child().custom_expect("No last child found".to_owned())
        }
        fn next_element_sibling_el(&self) -> Element {
            self.next_element_sibling().custom_expect("No next sibling found".to_owned())
        }
        fn prev_element_sibling_el(&self) -> Element {
            self.previous_element_sibling()
                .custom_expect("No prev sibling found".to_owned())
        }
        fn first_element_child_html(&self) -> HtmlElement {
            self.first_element_child()
                .custom_expect("No first child found".to_owned())
                .to_html()
        }
        fn last_element_child_html(&self) -> HtmlElement {
            self.last_element_child()
                .custom_expect("No last child found".to_owned())
                .to_html()
        }
        fn next_element_sibling_html(&self) -> HtmlElement {
            self.next_element_sibling()
                .custom_expect("No next sibling found".to_owned())
                .to_html()
        }
        fn prev_element_sibling_html(&self) -> HtmlElement {
            self.previous_element_sibling()
                .custom_expect("No prev sibling found".to_owned())
                .to_html()
        }
        fn get_attr_node(&self, value: &str) -> Attr {
            self.get_attribute_node(value).custom_expect("No attribute node found for : ".to_owned() + value.clone())
        }
        fn get_attr(&self, value: &str) -> String {
            self.get_attribute(value).custom_expect("No attribute found for : ".to_owned() + value.clone())
        }
        fn get_attr_ns(&self, namespace: &str, localname: &str) -> String {
            self.get_attribute_ns(Some(namespace), localname).custom_expect("There's been an error with get_attribute_namespace".to_owned())
        }
        fn toggle_attr(&self, value: &str) -> bool {
            self.toggle_attribute(value)
                .custom_expect("It's not possible toggle this attribute : ".to_owned() + value.clone())
        }
        fn get_pref(&self) -> String {
            self.prefix().custom_expect("No prefix found".to_owned())
        }
        fn insert_adj_el<'s, 'a>(&self, where_: &'s str, element: Element) -> Element {
            self.insert_adjacent_element(where_, &element)
                .custom_expect("It has not been possible insert element".to_owned())
                .custom_expect("There's been a problem".to_owned())
        }
        fn remove_attr(&self, value: &str) {
            self.remove_attribute(value).custom_expect("It's not possible remove attribute : ".to_owned() + value.clone())
        }
        fn has_match(&self, value: &str) -> bool {
            self.matches(value).custom_expect("There's not a match with value: ".to_owned()+ value.clone())
        }
        fn get_node_value(&self) -> String {
            self.node_value().custom_expect("No node value found".to_owned())
        }
        fn get_text_content(&self) -> String {
            self.text_content().custom_expect("No text content found".to_owned())
        }
        fn app_child(&self, node: Node) -> Node {
            self.append_child(&node).custom_expect("Failed to append child".to_owned())
        }
    }

    impl MinimalHtml for HtmlElement {
        fn to_el(&self) -> Element {
            self.clone()
                .dyn_into::<Element>()
                .custom_expect("It's not possible convert a Null Object to Element".to_owned())
        }
        fn offset_parent_el(&self) -> Element {
            self.offset_parent().custom_expect("No offset parent found".to_owned())
        }
        fn offset_parent_html(&self) -> HtmlElement {
            self.offset_parent_el()
                .to_html()
        }
        fn set_prop<'a, 's>(&self, property: &str, value: &'s str) {
            self.style().set_property(property, value).custom_expect("There's been a problem with setting property value : ".to_owned() + value.clone())
        }
        fn get_prop(&self, property: &str) -> String {
            self.style().get_property_value(property).custom_expect("There's been a problem with get the property value : ".to_owned() + property.clone())
        }
        fn remove_prop(&self, value: &str) -> String{
            self.style().remove_property(value).custom_expect("It's not possible remove this property : ".to_owned() + value.clone())
        }
        fn set_css(&self, value: &str) {
            self.style().set_css_text(value)
        }
        fn get_css(&self) -> String{
            self.style().css_text()
        }
    }

    impl MinimalList for NodeList {
        fn get_node(&self, index: u32) -> Node {
            self.item(index).custom_expect("There's not a Node in NodeList in position ".to_owned() + &index.to_string())
        }
        fn get_el(&self, index: u32) -> Element {
            self.get_node(index)
                .dyn_into::<Element>()
                .custom_expect("It's not possible get an Element of a Null Node".to_owned())
        }
        fn get_html(&self, index: u32) -> HtmlElement {
            self.get_node(index)
                .dyn_into::<HtmlElement>()
                .custom_expect("It's not possible get an HtmlElement of a Null Node".to_owned())
        }
        fn add_list_class(&self, value: &str) {
            for i in 0..self.length() {
                if !self.get_html(i).has_class(value) {
                    self.get_html(i).add_class(value);
                }
            }
        }
        fn remove_list_class(&self, value: &str) {
            for i in 0..self.length() {
                if self.get_html(i).has_class(value) {
                    self.get_html(i).remove_class(value);
                }
            }
        }
    }

    impl MinimalNode for Node{
        fn to_el(&self) -> Element {
            self.clone().dyn_into::<Element>().custom_expect("It's not possible convert a Null Node to Element".to_owned())
        }
        fn to_html(&self) -> HtmlElement {
            self.clone().dyn_into::<HtmlElement>().custom_expect("It's not possible convert a Null Node to HtmlElement".to_owned())
        }
    }
}
