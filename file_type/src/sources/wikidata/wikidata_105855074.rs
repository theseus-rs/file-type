use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855074: FileFormat = FileFormat {
    id: 105_855_074,
    source_type: SourceType::Wikidata,
    name: "Ashampoo Photo Commander Document",
    extensions: &["apcdoc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x00, 0x50, 0x00, 0x43, 0x00, 0x44, 0x00, 0x4F, 0x00, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
