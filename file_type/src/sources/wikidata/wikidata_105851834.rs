use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851834: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_834,
        source_type: SourceType::Wikidata,
        name: "SEAM 3D Project",
        extensions: &["s3d"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x45, 0x41, 0x4D, 0x20, 0x33, 0x44, 0x3A, 0x50, 0x72, 0x6F, 0x6A,
                        0x65, 0x63, 0x74, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
