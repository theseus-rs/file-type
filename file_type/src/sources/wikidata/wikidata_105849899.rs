use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849899: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_899,
        source_type: SourceType::Wikidata,
        name: "Citect Trend History data (v2)",
        extensions: &["001"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x49, 0x54, 0x45, 0x43, 0x54, 0x0A, 0x0D, 0x56, 0x45, 0x52, 0x53,
                        0x49, 0x4F, 0x4E, 0x20, 0x32, 0x0A, 0x0D, 0x54, 0x52, 0x45, 0x4E, 0x44,
                        0x20, 0x48, 0x49, 0x53, 0x54, 0x4F, 0x52, 0x59, 0x0A, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
