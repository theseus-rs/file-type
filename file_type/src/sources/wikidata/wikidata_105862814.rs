use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862814: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_814,
        source_type: SourceType::Wikidata,
        name: "MeshMixer scene",
        extensions: &["mix"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x40, 0x06, 0x00, 0x00, 0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65,
                        0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x3F,
                        0x3E, 0x0A, 0x3C, 0x53, 0x63, 0x65, 0x6E, 0x65, 0x4F, 0x62, 0x6A, 0x65,
                        0x63, 0x74, 0x73, 0x3E, 0x0A, 0x09, 0x3C, 0x52, 0x65, 0x63, 0x6F, 0x72,
                        0x64, 0x49, 0x44, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
