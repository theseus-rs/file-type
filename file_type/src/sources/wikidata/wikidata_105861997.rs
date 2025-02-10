use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861997: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_997,
        source_type: SourceType::Wikidata,
        name: "MATLAB Linux 64bit compiled function",
        extensions: &["mexa64"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7F, 0x45, 0x4C, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
