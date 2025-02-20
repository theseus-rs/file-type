use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852823: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_823,
        source_type: SourceType::Wikidata,
        name: "Spring Engine 3D model",
        extensions: &["s3o"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x70, 0x72, 0x69, 0x6E, 0x67, 0x20, 0x75, 0x6E, 0x69, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
