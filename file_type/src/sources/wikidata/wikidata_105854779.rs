use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854779: FileFormat = FileFormat {
    id: 105_854_779,
    source_type: SourceType::Wikidata,
    name: "Playmation Action",
    extensions: &["act"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x54, 0x49, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
