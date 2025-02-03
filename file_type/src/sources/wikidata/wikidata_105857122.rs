use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857122: FileFormat = FileFormat {
    id: 105_857_122,
    source_type: SourceType::Wikidata,
    name: "WinGraph Graph",
    extensions: &["grf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x20, 0x4D, 0x4A, 0x4D, 0x20, 0x11, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42,
                    0x11, 0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
