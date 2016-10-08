initSidebarItems({"enum":[["Align","The orientation of **Align**ment along some **Axis**."],["Axis","Represents either **Axis** in the 2-dimensional plane."],["Bordering","To be used as a parameter for defining the aesthetic of the widget border."],["Corner","Either of the four corners of a **Rect**."],["Dimension","The length of a **Widget** over either the *x* or *y* axes."],["Direction","Directionally positioned, normally relative to some other widget."],["Edge","Represents either the **Start** or **End** **Edge** of a **Range**."],["Place","Place the widget at a position on some other widget."],["Position","Some **Position** of some **Widget** along a single axis."]],"macro":[["builder_method!","A macro for simplifying implementation of methods for the `builder pattern`."],["builder_methods!","A macro to simplify implementation of \"builder-pattern\" methods."],["image_map!","A macro for simplifying the instantiation of an `image::Map`."],["widget_ids!","A macro used to generate a struct with a field for each unique identifier given. Each field can then be used to generate unique `widget::Id`s."],["widget_style!","A macro for vastly simplifying the definition and implementation of a widget's associated `Style` type."]],"mod":[["backend","Feature-gated, backend-specific functionality."],["color",""],["event","Contains all the structs and enums to describe all of the input events that `Widget`s can handle."],["graph","Conrod uses a directed acyclic graph to manage both storing widgets and describing their relationships."],["guide","**The Conrod Guide**"],["image","A type used to manage a user's image data and map them to `Image` widgets:"],["input","This module contains all the logic for handling input events and providing them to widgets."],["render","Conrod's generic graphics backend."],["text","Text layout logic."],["theme",""],["utils",""],["widget","Widgets are the core building blocks for every conrod user interface."]],"struct":[["Padding","The distance between the inner edge of a border and the outer edge of the inner content."],["Range","Some start and end position along a single axis."],["Rect","Defines a Rectangle's bounds across the x and y axes."],["Ui","`Ui` is the most important type within Conrod and is necessary for rendering and maintaining widget state. # Ui Handles the following: * Contains the state of all widgets which can be indexed via their widget::Id. * Stores rendering state for each widget until the end of each render cycle. * Contains the theme used for default styling of the widgets. * Maintains the latest user input state (for mouse and keyboard). * Maintains the latest window dimensions."],["UiBuilder","A constructor type for building a `Ui` instance with a set of optional parameters."],["UiCell","A wrapper around the `Ui` that restricts the user from mutating the `Ui` in certain ways while in the scope of the `Ui::set_widgets` function and within `Widget`s' `update` methods. Using the `UiCell`, users may access the `Ui` immutably (via `Deref`) however they wish, however they may only mutate the `Ui` via the `&mut self` methods provided by the `UiCell`."]],"trait":[["Borderable","Widgets that may display a border."],["Labelable","Widgets that may display some label."],["Positionable","Widgets that are positionable."],["Sizeable","Widgets that support different dimensions."]],"type":[["Depth","The depth at which the widget will be rendered."],["Dimensions","General use 2D spatial dimensions."],["FontSize","Font size used throughout Conrod."],["Margin","The margin for some `Place`ment on either end of an axis."],["Point","General use 2D spatial point."],["Scalar","An alias over the Scalar type used throughout Conrod."]]});