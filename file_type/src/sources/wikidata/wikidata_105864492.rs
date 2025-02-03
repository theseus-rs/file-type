use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864492: FileFormat = FileFormat {
    id: 105_864_492,
    source_type: SourceType::Wikidata,
    name: "Battlezone 2 game data package",
    extensions: &["pak"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x4F, 0x43, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
