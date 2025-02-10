use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1052000: FileFormat = FileFormat {
    id: 1_052_000,
    source_type: SourceType::Wikidata,
    name: "qcow",
    extensions: &["img", "qcow"],
    media_types: &["application/x-qemu-disk"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x51, 0x46, 0x49, 0xFB, 0x00, 0x00, 0x00, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
