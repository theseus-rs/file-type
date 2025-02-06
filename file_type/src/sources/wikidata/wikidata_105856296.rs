use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856296: FileFormat = FileFormat {
    id: 105_856_296,
    source_type: SourceType::Wikidata,
    name: "IBM SKF disk image",
    extensions: &["dsk"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xAA, 0x59, 0xF0, 0x00, 0x00, 0x02, 0x00, 0x00, 0x01, 0x00, 0x02, 0xE0, 0x00,
                    0x21, 0x00, 0x20, 0x0B, 0x09, 0x13, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
