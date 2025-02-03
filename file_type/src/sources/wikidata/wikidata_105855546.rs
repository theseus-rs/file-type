use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855546: FileFormat = FileFormat {
    id: 105_855_546,
    source_type: SourceType::Wikidata,
    name: "OFF geometry definition",
    extensions: &["off"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x46, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
