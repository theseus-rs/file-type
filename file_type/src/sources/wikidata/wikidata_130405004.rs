use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130405004: FileFormat = FileFormat {
    id: 130_405_004,
    source_type: SourceType::Wikidata,
    name: "Org file",
    extensions: &["org"],
    media_types: &["text/org"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23])],
            },
        }],
    }],
    related_formats: &[],
};
