use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855644: FileFormat = FileFormat {
    id: 105_855_644,
    source_type: SourceType::Wikidata,
    name: "Office Upgrade Control",
    extensions: &["opc"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x23, 0x23, 0x23, 0x20, 0x4F, 0x50, 0x43, 0x20, 0x46, 0x49, 0x4C, 0x45,
                    0x20, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
