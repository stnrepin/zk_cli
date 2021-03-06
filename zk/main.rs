use zk_core::app::ZkApp;
use zk_core::command_system::ZkCommandExecutionErrorKind;

fn main() {
    let mut output = std::io::stdout();
    let mut app = ZkApp::new(std::env::args().collect(), &mut output);
    let res = app.run();
    match res {
        Ok(_) => {},
        Err(ZkCommandExecutionErrorKind::ArgumentParsing(err)) => println!("{}", err),
        Err(ZkCommandExecutionErrorKind::UnknownCommand(mes)) => println!("error: {}", mes),
    }
}
