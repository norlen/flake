use crate::id::Flake;
use std::fmt;
use std::time::SystemTime;

fn get_millis(epoch: SystemTime) -> u64 {
    SystemTime::now().duration_since(epoch).unwrap().as_millis() as u64
}

#[derive(Debug, Clone)]
pub enum IdError {
    /// If a datacenter id does not fit within 5 bits.
    InvalidDatacenterId(i32),

    /// If a machien id does not fit within 5 bits.
    InvalidMachineId(i32),
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IdError::InvalidDatacenterId(id) => {
                write!(f, "datacenter id must fit within 5 bits, got {}", id)
            }
            IdError::InvalidMachineId(id) => {
                write!(f, "machine id must fit within 5 bits, got {}", id)
            }
        }
    }
}

/// Generator for snowflake IDs.
///
/// The timestamps contained in the ID can start at a custom epoch, which is supplied at startup.
/// Ensure this does not change if IDs have been generated, as a later epoch can create collisions.
///
/// Check the `Flake` struct for detailed information about the bit pattern. The pattern allows for
/// a maximum of 4096 snowflakes to be created each millisecond. If this limit is exceeded it creates
/// a snowflake for the next millisecond, this allows for short bursts of more than 4096 snowflakes
/// per second. However, these are created "in the future" so this should only happen in short bursts.
pub struct IdGenerator {
    /// Start of a custom epoch.
    epoch: SystemTime,

    /// Latest millisecond timestamp encountered.
    latest_millis: u64,

    /// Datacenter ID.
    datacenter_id: i32,

    /// Machine ID.
    machine_id: i32,

    /// Sequence number of the generated ID inside a single millisecond.
    sequence: i32,
}

impl IdGenerator {
    /// Creates a new `IdGenerator` with a custom epoch, datacenter id and machine id. The custom epoch should be the same
    /// for all instances, however the datacenter and machine id should be unique.
    ///
    /// Returns an error if `datacenter_id` or `machine_id` does not fit within 5 bits.
    pub fn new(epoch: SystemTime, datacenter_id: i32, machine_id: i32) -> Result<Self, IdError> {
        if datacenter_id >= Flake::DATACENTER_MAX {
            return Err(IdError::InvalidDatacenterId(datacenter_id));
        }
        if machine_id >= Flake::MACHINE_MAX {
            return Err(IdError::InvalidMachineId(machine_id));
        }

        Ok(Self {
            epoch,
            latest_millis: get_millis(epoch),
            datacenter_id,
            machine_id,
            sequence: 0,
        })
    }

    /// Creates a new `Flake` and returns it.
    ///
    /// If the sequence part of the flake exceeds 4096 the timestamp will contain the next millisecond.
    pub fn id(&mut self) -> Flake {
        let now = get_millis(self.epoch);
        if now > self.latest_millis {
            self.latest_millis = now;
            self.sequence = 0;
        }

        let flake = Flake::new(
            self.latest_millis,
            self.datacenter_id,
            self.machine_id,
            self.sequence,
        );
        self.sequence += 1;

        // If we overflow the sequence, take IDs from the next millisecond.
        if self.sequence >= Flake::SEQUENCE_MAX {
            self.latest_millis += 1;
            self.sequence = 0;
        }

        flake
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::UNIX_EPOCH;

    #[test]
    fn test() {
        let mut generator = IdGenerator::new(UNIX_EPOCH, 1, 1).unwrap();

        for _ in 0..10 {
            let snowflake = generator.id();
            println!("{:x?}", snowflake);
        }
    }
}
