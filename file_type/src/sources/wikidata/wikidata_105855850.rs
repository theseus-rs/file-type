use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855850: FileFormat = FileFormat {
    id: 105_855_850,
    puid: "wikidata/105855850",
    name: "Daisy Container File",
    extensions: &["dsy"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x41, 0x49, 0x53, 0x59, 0x20, 0x43, 0x4F, 0x4E, 0x54, 0x41, 0x49, 0x4E,
                    0x45, 0x52, 0x20, 0x46, 0x49, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
