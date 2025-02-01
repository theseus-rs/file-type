use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864779: FileFormat = FileFormat {
    id: 105_864_779,
    puid: "wikidata/105864779",
    name: "Borland C++ (OS/2) Project",
    extensions: &["prj"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x6F, 0x72, 0x6C, 0x61, 0x6E, 0x64, 0x20, 0x43, 0x2B, 0x2B, 0x20, 0x28,
                    0x4F, 0x53, 0x2F, 0x32, 0x29, 0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74,
                    0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
