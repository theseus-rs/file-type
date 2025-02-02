use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866606: FileFormat = FileFormat {
    id: 105_866_606,
    source_type: SourceType::Wikidata,
    name: "PowerPacker compressed (v2.0)",
    extensions: &["pp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x50, 0x32, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
