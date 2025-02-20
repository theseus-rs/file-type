use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856076: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_076,
        source_type: SourceType::Wikidata,
        name: "ParaCAD+ Drawing (v2)",
        extensions: &["drg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x70, 0x61, 0x72, 0x61, 0x43, 0x41, 0x44, 0x2B, 0x20, 0x44, 0x72, 0x61,
                        0x77, 0x69, 0x6E, 0x67, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x28, 0x76,
                        0x32, 0x29, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
