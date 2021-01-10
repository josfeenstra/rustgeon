


// Result and Some error handling

fn test() -> Option<u32>
{
    if false {
        Some(32)
    } else {
        None
    }
}


fn something() -> Result<u32, String>
{
    if false
    {
        Ok(32)
    }
    else
    {
        Err(String::from("not good"))
    }
}
