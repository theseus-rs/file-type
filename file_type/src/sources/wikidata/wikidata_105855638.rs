use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855638: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_638,
        source_type: SourceType::Wikidata,
        name: "Oberon V3 Symbol data",
        extensions: &["sym"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x59, 0x4D, 0x3A, 0x30, 0x30, 0x31, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
