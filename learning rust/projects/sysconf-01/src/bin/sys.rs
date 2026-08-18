#[derive(Debug)]
#[allow(unused)]
enum Optmode {
    ReadWrite,
    ReadOnly,
    WriteOnly,
}
#[derive(Debug)]
#[allow(unused)]
struct System {
    name: String,
    cpu: String,
    core_cnt: u32,
    mem: u32,
    sys_enabled: bool,
    opt_mode: Optmode,
}
fn main() {
    let memopt: [u32; 4] = [4, 8, 16, 32];
    let win: System = System {
        name: "Windows".to_string(),
        cpu: "I3 8100".to_string(),
        core_cnt: 4,
        mem: memopt[1],
        sys_enabled: true,
        opt_mode: Optmode::ReadWrite,
    };
    let linux: System = System {
        name: "Linux".to_string(),
        cpu: "I9 9800k".to_string(),
        core_cnt: 16,
        mem: memopt[3],
        sys_enabled: true,
        opt_mode: Optmode::ReadWrite,
    };

    let mut systems = (win, linux);

    println!("{:#?}", systems.0);
    println!("{:#?}", systems.1);
    println!(
        r#"
        Lets update Windows system to:
        memory => 16 gb
        cpu => i7 8700
            Hence core count will be 8
        Along the update lets disable the system
        and Make it Read Only system.
        "#
    );
    systems.0.cpu = "i7 8700".to_string();
    systems.0.core_cnt = 8;
    systems.0.mem = memopt[2];
    systems.0.sys_enabled = false;
    systems.0.opt_mode = Optmode::ReadOnly;
    println!("{:#?}", systems.0);
}
