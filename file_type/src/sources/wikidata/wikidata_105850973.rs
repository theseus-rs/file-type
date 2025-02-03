use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850973: FileFormat = FileFormat {
    id: 105_850_973,
    source_type: SourceType::Wikidata,
    name: "GameCube THP video",
    extensions: &["thp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x48, 0x50, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
