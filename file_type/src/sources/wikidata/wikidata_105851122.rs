use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851122: FileFormat = FileFormat {
    id: 105_851_122,
    puid: "wikidata/105851122",
    name: "ArtCAM Toolpath Template",
    extensions: &["tpl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFE, 0xFF, 0x18, 0x41, 0x00, 0x72, 0x00, 0x74, 0x00, 0x43, 0x00, 0x41,
                    0x00, 0x4D, 0x00, 0x20, 0x00, 0x54, 0x00, 0x6F, 0x00, 0x6F, 0x00, 0x6C, 0x00,
                    0x70, 0x00, 0x61, 0x00, 0x74, 0x00, 0x68, 0x00, 0x20, 0x00, 0x54, 0x00, 0x65,
                    0x00, 0x6D, 0x00, 0x70, 0x00, 0x6C, 0x00, 0x61, 0x00, 0x74, 0x00, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
