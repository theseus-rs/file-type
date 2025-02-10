use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850137: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_137,
        source_type: SourceType::Wikidata,
        name: "Back-It device definition (v4)",
        extensions: &["cfg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x61, 0x7A, 0x65, 0x6C, 0x6C, 0x65, 0x20, 0x53, 0x79, 0x73, 0x74,
                        0x65, 0x6D, 0x73, 0x0D, 0x0A, 0x20, 0x42, 0x61, 0x63, 0x6B, 0x69, 0x74,
                        0x20, 0x34, 0x20, 0x0D, 0x0A, 0x20, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65,
                        0x20, 0x44, 0x65, 0x66, 0x69, 0x6E, 0x69, 0x74, 0x69, 0x6F, 0x6E, 0x20,
                        0x66, 0x69, 0x6C, 0x65, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
