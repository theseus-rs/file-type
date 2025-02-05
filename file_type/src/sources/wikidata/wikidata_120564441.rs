use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120564441: FileFormat = FileFormat {
    id: 120_564_441,
    source_type: SourceType::Wikidata,
    name: "Express Publisher Document (Windows)",
    extensions: &["ewd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xCA, 0xFE, 0xBE, 0xEF])],
            },
        }],
    }],
    related_formats: &[],
};
