use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858707: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_707,
        source_type: SourceType::Wikidata,
        name: "Blu-ray Disc Movie Information",
        extensions: &["bdm", "bdmv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x4F, 0x42, 0x4A, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
