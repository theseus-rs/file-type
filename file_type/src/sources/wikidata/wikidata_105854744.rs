use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854744: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_744,
        source_type: SourceType::Wikidata,
        name: "Digital Speech Standard audio (v2)",
        extensions: &["dss"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x02, 0x64, 0x73, 0x73])],
                },
            }],
        }],
        related_formats: &[],
    },
};
