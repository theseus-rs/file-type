use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859596: FileFormat = FileFormat {
    id: 105_859_596,
    source_type: SourceType::Wikidata,
    name: "Nancy Codec video",
    extensions: &["noa"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD2, 0x4E, 0x4F, 0x41, 0x0F, 0x0C, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
