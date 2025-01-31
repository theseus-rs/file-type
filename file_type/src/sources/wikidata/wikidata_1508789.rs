use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1508789: FileFormat = FileFormat {
    id: 1_508_789,
    puid: "wikidata/1508789",
    name: "Windows Contacts",
    extensions: &["contact"],
    media_types: &["text/x-ms-contact"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x65, 0x6E, 0x63, 0x6F, 0x64, 0x69,
                    0x6E, 0x67, 0x3D, 0x22, 0x55, 0x54, 0x46, 0x2D, 0x38, 0x22, 0x3F, 0x3E, 0x0D,
                    0x0A, 0x3C, 0x63, 0x3A, 0x63, 0x6F, 0x6E, 0x74, 0x61, 0x63, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
