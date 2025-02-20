use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28756583: FileType = FileType {
    file_format: &FileFormat {
        id: 28_756_583,
        source_type: SourceType::Wikidata,
        name: "Fountain script",
        extensions: &["fountain", "spmd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x69, 0x74, 0x6C, 0x65, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
