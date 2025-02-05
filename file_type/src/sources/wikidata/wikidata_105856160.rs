use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856160: FileFormat = FileFormat {
    id: 105_856_160,
    source_type: SourceType::Wikidata,
    name: "Diagram! Palette",
    extensions: &["dpa"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x04, 0x0B, 0x74, 0x79, 0x70, 0x65, 0x64, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6D,
                    0x81, 0x03, 0xA2, 0x84, 0x01, 0x69, 0x01, 0x84, 0x01, 0x40, 0x84, 0x84, 0x84,
                    0x09, 0x50, 0x72, 0x69, 0x6E, 0x74, 0x49, 0x6E, 0x66, 0x6F, 0x00, 0x84, 0x84,
                    0x06, 0x4F, 0x62, 0x6A, 0x65, 0x63, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
