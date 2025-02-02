use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757724: FileFormat = FileFormat {
    id: 28_757_724,
    source_type: SourceType::Wikidata,
    name: "GDIFF",
    extensions: &["gdiff"],
    media_types: &["application/gdiff"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD1, 0xFF, 0xD1, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
