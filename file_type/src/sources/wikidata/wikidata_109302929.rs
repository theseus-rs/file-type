use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109302929: FileFormat = FileFormat {
    id: 109_302_929,
    puid: "wikidata/109302929",
    name: "Asymetrix Compel Presentation, version 2",
    extensions: &["cpl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x03, 0x4A, 0x42, 0x4F, 0x4F, 0xF5, 0x3C, 0x55,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
