use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864408: FileFormat = FileFormat {
    id: 105_864_408,
    source_type: SourceType::Wikidata,
    name: "Portable Bridge Notation (gen)",
    extensions: &["pbn"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x20, 0x50, 0x42, 0x4E, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
