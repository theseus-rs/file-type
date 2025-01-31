use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867576: FileFormat = FileFormat {
    id: 105_867_576,
    puid: "wikidata/105867576",
    name: "DOT_MAGIC! NLQ font",
    extensions: &["nlq"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x4F, 0x54, 0x5F, 0x4D, 0x41, 0x47, 0x49, 0x43, 0x20, 0x4E, 0x4C, 0x51,
                    0x20, 0x46, 0x4F, 0x4E, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
