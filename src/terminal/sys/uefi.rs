use crate::Error;

pub(crate) fn is_raw_mode_enabled() -> Result<bool, Error> {
    Ok(false)
}

pub(crate) fn enable_raw_mode() -> Result<(), Error> {
    Ok(())
}

pub(crate) fn disable_raw_mode() -> Result<(), Error> {
    Ok(())
}

pub(crate) fn size() -> Result<(usize, usize), Error> {
    Ok((80, 25))
}
