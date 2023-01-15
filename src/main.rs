mod application;

use application::*;

fn main() -> Result<(), confy::ConfyError> {
    app()
}