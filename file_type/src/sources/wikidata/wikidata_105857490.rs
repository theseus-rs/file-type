use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857490: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_490,
        source_type: SourceType::Wikidata,
        name: "3D Construction Kit Shape data (v2.x)",
        extensions: &["3sd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x74, 0x61, 0x6E, 0x64, 0x61, 0x72, 0x64, 0x20, 0x43, 0x52, 0x45,
                        0x41, 0x54, 0x49, 0x4F, 0x4E, 0x20, 0x53, 0x68, 0x61, 0x70, 0x65, 0x20,
                        0x66, 0x69, 0x6C, 0x65, 0x3A, 0x20, 0x52, 0x65, 0x6C, 0x65, 0x61, 0x73,
                        0x65, 0x20, 0x56, 0x32, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
