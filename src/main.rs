use hyprland::data::*;
use hyprland::dispatch::Dispatch;
use hyprland::dispatch::DispatchType;
use hyprland::dispatch::WindowIdentifier;
use hyprland::prelude::*;
//use hyprland::dispatch::*;
use hyprland::shared::Address;
use hyprland::Result;

struct WindowsChange {
    origin: i32,
    address: Address,
}

fn main() -> Result<()> {
    let traslate = match active_workspaces() {
        Ok(r) => r,
        Err(e) => panic!("Error: {}", e)
    };

    let _ = Dispatch::call(DispatchType::MoveToWorkspace(hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(traslate[1].origin), Some(WindowIdentifier::Address(traslate[0].address.clone()) )));
    let _ = Dispatch::call(DispatchType::MoveToWorkspace(hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(traslate[0].origin), Some(WindowIdentifier::Address(traslate[1].address.clone()) )));

    Ok(())
}

fn active_workspaces() -> Result<Vec<WindowsChange>> {
    let aw = match Workspace::get_active() {
        Ok(o) => o,
        Err(e) => panic!("Error: {}", e),
    };

    let second_mon: Vec<Monitor> = Monitors::get()?
        .into_iter()
        .filter(|x| x.name != aw.monitor)
        .collect();

    let mut change: Vec<WindowsChange> = Vec::new();

    change.push(WindowsChange {
            origin: aw.id,
            address: aw.last_window.clone(),
        });

    let held: Vec<_> = Workspaces::get()?.into_iter().by_ref().collect();

    let tmp: Vec<Workspace> = held
        .clone()
        .into_iter()
        .filter(|x| x.id == second_mon[0].active_workspace.id && x.monitor != aw.monitor)
        .collect();

    change.push(WindowsChange {
        origin: second_mon[0].active_workspace.id,
        address: tmp[0].last_window.clone(),
    });

    Ok(change)
}
