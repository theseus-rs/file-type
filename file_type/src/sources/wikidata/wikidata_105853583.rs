use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853583: FileFormat = FileFormat {
    id: 105_853_583,
    source_type: SourceType::Wikidata,
    name: "z-Tree Treatment",
    extensions: &["ztt"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x08, 0x43, 0x50, 0x47, 0x58, 0x47, 0x61, 0x6D, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
