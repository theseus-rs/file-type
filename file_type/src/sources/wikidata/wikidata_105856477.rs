use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856477: FileFormat = FileFormat {
    id: 105_856_477,
    source_type: SourceType::Wikidata,
    name: "WordPerfect document (Amiga)",
    extensions: &["wp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x80, 0x80])],
            },
        }],
    }],
    related_formats: &[],
};
