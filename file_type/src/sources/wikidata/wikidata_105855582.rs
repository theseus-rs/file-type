use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855582: FileFormat = FileFormat {
    id: 105_855_582,
    source_type: SourceType::Wikidata,
    name: "Meridian OPL4 soundbank",
    extensions: &["opl"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x4D, 0x45, 0x52, 0x49, 0x44, 0x49, 0x41, 0x4E, 0x20, 0x4F, 0x50, 0x4C,
                    0x34, 0x20, 0x53, 0x4F, 0x55, 0x4E, 0x44, 0x42, 0x41, 0x4E, 0x4B, 0x25,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
