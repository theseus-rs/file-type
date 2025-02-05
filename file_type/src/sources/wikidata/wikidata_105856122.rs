use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856122: FileFormat = FileFormat {
    id: 105_856_122,
    source_type: SourceType::Wikidata,
    name: "Delphi Project",
    extensions: &["dproj"],
    media_types: &["text/xml"],
    signatures: &[Signature {
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
