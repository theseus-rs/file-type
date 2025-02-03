use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855072: FileFormat = FileFormat {
    id: 105_855_072,
    source_type: SourceType::Wikidata,
    name: "Savage Warriors Animation",
    extensions: &["anm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x48, 0x43, 0x4B, 0x00, 0x01, 0x03, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
