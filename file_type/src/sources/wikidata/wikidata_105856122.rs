use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856122: FileFormat = FileFormat {
    id: 105_856_122,
    source_type: SourceType::Wikidata,
    name: "Delphi Project",
    extensions: &["dproj"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF])],
            },
        }],
    }],
    related_formats: &[],
};
