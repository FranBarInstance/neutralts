
// Inherit, macro: new_child_parse
// -------------------------------
// Inheritance is implemented with this macro. It is also used for the inheritance
// of the application itself.
//
//
// Block level scope example:
//    {:code; <--------------------------.
//        {:* block *:}                  |<---- Block
//        {:param; name >> value :} <----|----- Set "name" for this block and its children
//        {:param; name :} <-------------|----- "name" has the value "value".
//        {:code;                        |
//            {:* child block *:}        |
//            {:param; name :} <---------|----- "name" has the value "value".
//        :}                             |
//    :} <-------------------------------·
//    {:param; name :} <----------------------- outside block, no value or a previous value if any.
//
//
// "include" has a block scope, then:
//    {:code;
//        {:* include for set "snippet-name" *:}
//        {:include; snippet.ntpl :}
//        {:snippet; snippet-name :} {:* Ok, "snippet-name" is set *:}
//    :}
//    {:snippet; snippet-name :} {:* Ko, "snippet-name" is not set *:}
//
// The modifier scope (+) adds the scope to the current level to make it possible to do this:
//    {:+bool; something >>
//        {:include; snippet.ntpl :}
//    :}
//    {:snippet; snippet-name :} {:* Ok, "snippet-name" is set *:}
//
#[macro_use]
mod macros {
    macro_rules! new_child_parse {
        ($self:expr, $source:expr, $scope:expr) => {{
            let mut child_inherit = $self.inherit.clone();
            let shared = &mut $self.shared;

            //  "bif.alias" is used and not "bif.name" because in "var" or "unprintable"
            // its name is an empty string and could have more meanings.
            child_inherit.alias = $self.alias.clone();

            if !$self.file_path.is_empty() {
                child_inherit.current_file = $self.file_path.clone();
            }

            if !$self.dir.is_empty() {
                child_inherit.current_dir = $self.dir.clone();
            }

            // Create a new version of the schema if mod_scope
            // This is necessary because indirections are used,
            // and create_block_schema takes care of that.
            if $scope {
                $self.inherit.create_block_schema(shared);
            }

            let mut block = $crate::block_parser::BlockParser::new(shared, &child_inherit);
            let code = block.parse($source, $self.only);

            // Update this block with the data generated in the child
            if $scope {
                // el código que estaba aquí lo he movido a la función
                // update_indir para evitar un error de prestamo.
                block.update_indir(&$self.inherit.indir);
            }

            code
        }};
    }
}
