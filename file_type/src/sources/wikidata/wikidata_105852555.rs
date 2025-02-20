use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852555: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_555,
        source_type: SourceType::Wikidata,
        name: "Spazio3D drawing",
        extensions: &["s3d"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3A, 0x01, 0x01, 0x00, 0x00, 0x0D, 0x00, 0x00, 0x00, 0x42, 0x72, 0x61,
                        0x69, 0x6E, 0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x11, 0x00,
                        0x00, 0x00, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x20, 0x53, 0x70,
                        0x61, 0x7A, 0x69, 0x6F, 0x20, 0x33, 0x44, 0x06, 0x00, 0x00, 0x00, 0x76,
                        0x65, 0x72, 0x32, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
