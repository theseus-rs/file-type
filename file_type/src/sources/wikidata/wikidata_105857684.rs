use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857684: FileFormat = FileFormat {
    id: 105_857_684,
    source_type: SourceType::Wikidata,
    name: "KISSSlicer printer profile",
    extensions: &["ini"],
    media_types: &["text/ini"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0D, 0x0A, 0x5B])],
            },
        }],
    }],
    related_formats: &[],
};
