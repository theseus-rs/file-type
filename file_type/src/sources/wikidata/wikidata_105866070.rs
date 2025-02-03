use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866070: FileFormat = FileFormat {
    id: 105_866_070,
    source_type: SourceType::Wikidata,
    name: "Order of War game data archive",
    extensions: &["pkg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x00, 0x4B, 0x00, 0x47, 0x00, 0x5F, 0x00, 0x46, 0x00, 0x49, 0x00, 0x4C,
                    0x00, 0x45, 0x00, 0x5F, 0x00, 0x56, 0x00, 0x45, 0x00, 0x52, 0x00, 0x53, 0x00,
                    0x49, 0x00, 0x4F, 0x00, 0x4E, 0x00, 0x3A, 0x00, 0x30, 0x00, 0x30, 0x00, 0x30,
                    0x00, 0x34, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
