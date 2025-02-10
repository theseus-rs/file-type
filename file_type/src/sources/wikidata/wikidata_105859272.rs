use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859272: FileFormat = FileFormat {
    id: 105_859_272,
    source_type: SourceType::Wikidata,
    name: "IBM Printer Page Segment",
    extensions: &["pse", "pseg"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x00, 0x16, 0x00, 0x00, 0x01, 0x00, 0x00, 0x50, 0xE6, 0x99, 0x89, 0xA3,
                    0xA3, 0x85, 0x95, 0x40, 0x82, 0xA8, 0x40, 0xC7, 0xC2, 0xD4, 0x5A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
