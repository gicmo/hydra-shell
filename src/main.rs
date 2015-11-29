use std::io::Write;
use std::process::Command;

fn translate_path(vpath: &str) -> String {
    let exe = std::env::current_exe().unwrap();
    let cwd = exe.parent().unwrap();

    let to_trim: &[char] = &[' ', '\''];
    let mut repo_path = String::from(cwd.to_str().unwrap());
    repo_path.push_str(vpath.trim_matches(to_trim));

    if !vpath.ends_with(".git") {
        repo_path.push_str(".git");
    }

    repo_path
}

fn git_upload_pack(args: &str, uid: &str) {

    let ref repo_dir = &*translate_path(args);

    {
        let mut stderr = std::io::stderr();
        let _ = stderr.write_fmt(format_args!("[D] git-upload-pack @ {} -> {} [{}] \n",
                                       args, repo_dir, uid));
    }

    let mut child = Command::new("git-upload-pack")
                            .arg(repo_dir)
                            .spawn()
                            .unwrap_or_else(|e| { panic!("[E] executing child ({})", e) });

    let status = child.wait().unwrap_or_else(|e| { panic!("[E] waiting on child ({})", e) });

    assert!(status.success());
}

fn main() {
    let v: Vec<String> = std::env::args().collect();
    let ssh_cmd = std::env::var("SSH_ORIGINAL_COMMAND");

    if v.len() < 2 {
        panic!("[E] :( (no arg)");
    }

    let ref uid = v[1];

    let cmdline = match ssh_cmd {
        Ok(c) => c,
        Err(_) => panic!("[E] :( (no cmd)")
    };

    let pos = cmdline.find(' ').unwrap_or(cmdline.len());
    let (gitcmd, gitarg) = cmdline.split_at(pos);

    // Set the current working dir to the parent of the executable
    let exe = std::env::current_exe().unwrap();
    let cwd = exe.parent().unwrap();
    assert!(std::env::set_current_dir(&cwd).is_ok());

    {
        let mut stderr = std::io::stderr();
        let _ = stderr.write_fmt(format_args!("[D] CWD: {}\n", cwd.display()));
        let _ = stderr.write_fmt(format_args!("[D] {} [{}, {}] {}\n", cmdline, gitcmd, gitarg, uid));
    }

    match gitcmd {
        "git-upload-pack" => git_upload_pack(gitarg, uid),
        _ => panic!("[E] unhandled command")
    }

}
