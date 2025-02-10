use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853020: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_020,
        source_type: SourceType::Wikidata,
        name: "SEM Snapshot",
        extensions: &["sem"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x05, 0x53, 0x50, 0x45, 0x43, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
