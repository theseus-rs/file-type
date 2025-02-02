use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851543: FileFormat = FileFormat {
    id: 105_851_543,
    source_type: SourceType::Wikidata,
    name: "KC85 Tape image",
    extensions: &["tap"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xC3, 0x4B, 0x43, 0x2D, 0x54, 0x41, 0x50, 0x45, 0x20, 0x62, 0x79, 0x20, 0x41,
                    0x46, 0x2E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
