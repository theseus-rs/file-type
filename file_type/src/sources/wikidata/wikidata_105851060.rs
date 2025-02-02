use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851060: FileFormat = FileFormat {
    id: 105_851_060,
    source_type: SourceType::Wikidata,
    name: "Trackerpacker 3 Music",
    extensions: &["tp3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x50, 0x4C, 0x58, 0x5F, 0x54, 0x50, 0x33,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
