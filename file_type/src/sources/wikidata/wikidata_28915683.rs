use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28915683: FileType = FileType {
    file_format: &FileFormat {
        id: 28_915_683,
        source_type: SourceType::Wikidata,
        name: "Apache Parquet",
        extensions: &["parquet"],
        media_types: &["application/vnd.apache.parquet"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x41, 0x52, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
