use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857604: FileFormat = FileFormat {
    id: 105_857_604,
    source_type: SourceType::Wikidata,
    name: "Virtual Floppy Disk image (v1.00)",
    extensions: &["fdd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x46, 0x44, 0x31, 0x2E, 0x30, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
