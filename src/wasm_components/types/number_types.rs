/*
 * Data types
 * uintN: unsigned integer of N bits. little endian
 * varuintN: unsigned variable-length integer of N bits. leb128
 * varintN: signed variable-length integer of N bits. leb128
 *
 * Wasmバイナリの内部表現で用いられる数値表現のタイプ
 * c.f. leb128
 */

pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;

pub type VarUInt1 = u8;
pub type VarUInt7 = u8;
pub type VarUInt32 = u32;

pub type VarInt7 = i8;
pub type VarInt32 = i32;
pub type VarInt64 = i64;
