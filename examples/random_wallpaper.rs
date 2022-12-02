use kabegami::linux::Linux;
use kabegami::Platform;
use kabegami::error::Result;

fn main() -> Result<()> {
    let linux = Linux::new()?;
    let list_files = {
        let path = std::env::args().nth(1).expect("no path given");
        let mut paths = Vec::new();
        let dir = std::fs::read_dir(path)?;
        for entry in dir {
            let entry = entry?;
            paths.push(entry.path())
        };
        paths
    };
    linux.set_random_bg(list_files.into_iter())
}
