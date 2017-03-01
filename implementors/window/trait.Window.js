(function() {var implementors = {};
implementors["conrod"] = ["impl <a class='trait' href='window/trait.Window.html' title='window::Window'>Window</a> for <a class='struct' href='glutin_window/struct.GlutinWindow.html' title='glutin_window::GlutinWindow'>GlutinWindow</a>","impl&lt;W&gt; <a class='trait' href='window/trait.Window.html' title='window::Window'>BasicWindow</a> for <a class='struct' href='conrod/backend/piston/window/struct.Window.html' title='conrod::backend::piston::window::Window'>Window</a>&lt;W&gt; <span class='where'>where W: <a class='trait' href='window/trait.Window.html' title='window::Window'>BasicWindow</a></span>",];
implementors["glutin_window"] = ["impl <a class='trait' href='window/trait.Window.html' title='window::Window'>Window</a> for <a class='struct' href='glutin_window/struct.GlutinWindow.html' title='glutin_window::GlutinWindow'>GlutinWindow</a>",];
implementors["poke_a_mango"] = ["impl&lt;W&gt; <a class='trait' href='window/trait.Window.html' title='window::Window'>Window</a> for <a class='struct' href='conrod/backend/piston/window/struct.Window.html' title='conrod::backend::piston::window::Window'>Window</a>&lt;W&gt; <span class='where'>where W: <a class='trait' href='window/trait.Window.html' title='window::Window'>Window</a></span>","impl <a class='trait' href='window/trait.Window.html' title='window::Window'>Window</a> for <a class='struct' href='glutin_window/struct.GlutinWindow.html' title='glutin_window::GlutinWindow'>GlutinWindow</a>",];
implementors["window"] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
