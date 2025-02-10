use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858326: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_326,
        source_type: SourceType::Wikidata,
        name: "easyHDR 3 Settings",
        extensions: &["ehsx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x65, 0x61, 0x73, 0x79, 0x48, 0x44, 0x52, 0x73, 0x65, 0x74, 0x74,
                        0x69, 0x6E, 0x67, 0x73, 0x20, 0x50, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D,
                        0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
