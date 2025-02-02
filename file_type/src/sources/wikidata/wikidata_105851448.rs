use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851448: FileFormat = FileFormat {
    id: 105_851_448,
    source_type: SourceType::Wikidata,
    name: "Jane's Longbow 2 game data archive",
    extensions: &["tre"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x4B, 0x4E, 0x4B, 0x54, 0x52, 0x45, 0x45, 0x30, 0x31, 0x30, 0x41,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
