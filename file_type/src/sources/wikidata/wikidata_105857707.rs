use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857707: FileFormat = FileFormat {
    id: 105_857_707,
    puid: "wikidata/105857707",
    name: "North Star Horizon Hard Disk image",
    extensions: &["nhd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0xFF, 0x54, 0x52, 0x41, 0x4E, 0x53, 0x49, 0x45, 0x4E, 0x54, 0x2C, 0x53,
                    0x59, 0x53, 0x54, 0x45, 0x4D, 0x2C, 0x31, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
