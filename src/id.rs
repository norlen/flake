/// A (hopefully) unique snowflake.
///
/// The flake consists of a timestamp, datacenter id, machine id and a sequence number.
#[derive(Clone, Copy, Debug)]
pub struct Flake(pub u64);

impl Flake {
    // Shifts and masks for the fields contained in the snowflake.
    pub(crate) const TIMESTAMP_SHIFT: u64 = 22;

    pub(crate) const DATACENTER_SHIFT: u64 = 17;
    pub(crate) const DATACENTER_MASK: u64 = 0x3E0000;

    pub(crate) const MACHINE_SHIFT: u64 = 12;
    pub(crate) const MACHINE_MASK: u64 = 0x1F000;

    pub(crate) const SEQUENCE_MASK: u64 = 0xFFF;
    pub(crate) const SEQUENCE_MAX: i32 = 4096;

    /// Creates a new flake.
    ///
    /// The timestamp must fit within 42 bits, the datacenter and machine id within 5 bits, and the sequence within 12 bits.
    /// This is not checked so it is up to the caller to ensure this.
    pub fn new(timestamp: u64, datacenter_id: i32, machine_id: i32, sequence: i32) -> Self {
        let milliseconds = timestamp << Self::TIMESTAMP_SHIFT;
        let datacenter_id = (datacenter_id as u64) << Self::DATACENTER_SHIFT;
        let machine_id = (machine_id as u64) << Self::MACHINE_SHIFT;
        let sequence = sequence as u64;

        Self(milliseconds | datacenter_id | machine_id | sequence)
    }

    /// Gets the timestamp contained in the snowflake. This can be at most 42 bits.
    pub fn timestamp(&self) -> u64 {
        self.0 >> Self::TIMESTAMP_SHIFT
    }

    /// Gets the datacenter ID contained in the snowflake. This can be at most 5 bits.
    pub fn datacenter_id(&self) -> u64 {
        (self.0 & Self::DATACENTER_MASK) >> Self::DATACENTER_SHIFT
    }

    /// Gets the machine ID contained in the snowflake. This can be at most 5 bits.
    pub fn machine_id(&self) -> u64 {
        (self.0 & Self::MACHINE_MASK) >> Self::MACHINE_SHIFT
    }

    /// Gets the sequence contained in the snowflake. This can be at most 12 bits.
    pub fn sequence(&self) -> u64 {
        self.0 & Self::SEQUENCE_MASK
    }
}
