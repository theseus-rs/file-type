use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29896356: FileFormat = FileFormat {
    id: 29_896_356,
    source_type: SourceType::Wikidata,
    name: "EMBL file format",
    extensions: &["embl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x44, 0x20, 0x20, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
