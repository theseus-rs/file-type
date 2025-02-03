use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855150: FileFormat = FileFormat {
    id: 105_855_150,
    source_type: SourceType::Wikidata,
    name: "Font Specifications (with rem)",
    extensions: &["fontspec"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x25])],
            },
        }],
    }],
    related_formats: &[],
};
