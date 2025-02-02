use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861511: FileFormat = FileFormat {
    id: 105_861_511,
    source_type: SourceType::Wikidata,
    name: "LMMC encoded router config",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x4D, 0x4D, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
