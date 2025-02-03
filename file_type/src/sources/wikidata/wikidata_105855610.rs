use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855610: FileFormat = FileFormat {
    id: 105_855_610,
    source_type: SourceType::Wikidata,
    name: "Notaro document",
    extensions: &["oad"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7B, 0x5C])],
            },
        }],
    }],
    related_formats: &[],
};
