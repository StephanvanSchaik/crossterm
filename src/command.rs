use core::fmt;

use crate::Error;

mod sys;

/// An interface for a command that performs an action on the terminal.
///
/// Crossterm provides a set of commands,
/// and there is no immediate reason to implement a command yourself.
/// In order to understand how to use and execute commands,
/// it is recommended that you take a look at [Command API](./index.html#command-api) chapter.
pub trait Command {
    /// Write an ANSI representation of this command to the given writer.
    /// An ANSI code can manipulate the terminal by writing it to the terminal buffer.
    /// However, only Windows 10 and UNIX systems support this.
    ///
    /// This method does not need to be accessed manually, as it is used by the crossterm's [Command API](./index.html#command-api)
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result;

    /// Execute this command.
    ///
    /// Windows versions lower than windows 10 do not support ANSI escape codes,
    /// therefore a direct WinAPI call is made.
    ///
    /// This method does not need to be accessed manually, as it is used by the crossterm's [Command API](./index.html#command-api)
    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<(), Error>;

    /// Returns whether the ANSI code representation of this command is supported by windows.
    ///
    /// A list of supported ANSI escape codes
    /// can be found [here](https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences).
    #[cfg(windows)]
    fn is_ansi_code_supported(&self) -> bool {
        super::ansi_support::supports_ansi()
    }
}

impl<T: Command + ?Sized> Command for &T {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        (**self).write_ansi(f)
    }

    #[inline]
    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<(), Error> {
        T::execute_winapi(self)
    }

    #[cfg(windows)]
    #[inline]
    fn is_ansi_code_supported(&self) -> bool {
        T::is_ansi_code_supported(self)
    }
}

/// An interface for types that can queue commands for further execution.
pub trait QueueableCommand {
    /// Queues the given command for further execution.
    fn queue(&mut self, command: impl Command) -> Result<&mut Self, Error>;
}

/// An interface for types that can directly execute commands.
pub trait ExecutableCommand {
    /// Executes the given command directly.
    fn execute(&mut self, command: impl Command) -> Result<&mut Self, Error>;
}

/// An interface for types that support synchronized updates.
pub trait SynchronizedUpdate {
    /// Performs a set of actions against the given type.
    fn sync_update<T>(&mut self, operations: impl FnOnce(&mut Self) -> T) -> Result<T, Error>;
}

/// Executes the ANSI representation of a command, using the given `fmt::Write`.
pub(crate) fn execute_fmt(f: &mut impl fmt::Write, command: impl Command) -> fmt::Result {
    #[cfg(windows)]
    if !command.is_ansi_code_supported() {
        return command.execute_winapi().map_err(|_| fmt::Error);
    }

    command.write_ansi(f)
}
