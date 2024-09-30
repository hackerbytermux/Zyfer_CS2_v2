
pub unsafe fn find_process(process_name: &str) -> Result<u32, windows::core::Error> // Change type to u32
{
    let mut system = sysinfo::System::new();
    system.refresh_processes(sysinfo::ProcessesToUpdate::All);
    let process = system.processes_by_name(process_name.as_ref()).next();

    if let Some(process) = process
    {
        return Ok(process.pid().as_u32());
    }
    else
    {
        return Err(windows::core::Error::from_win32());
    }
}