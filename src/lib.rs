pub mod state;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod utils;

use processor::process_instruction;
use solana_program::entrypoint;

entrypoint!(process_instruction);
//Just another new line for testing purposes


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
