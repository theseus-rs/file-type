use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861409: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_409,
        source_type: SourceType::Wikidata,
        name: "SuperKey keyboard Layout",
        extensions: &["lay"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x2A, 0x2A, 0x20, 0x4C, 0x41, 0x59, 0x4F, 0x55, 0x54, 0x20, 0x46,
                        0x49, 0x4C, 0x45, 0x20, 0x2A, 0x2A, 0x2A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
