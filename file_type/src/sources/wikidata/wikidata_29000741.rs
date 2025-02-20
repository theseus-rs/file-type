use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000741: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_741,
        source_type: SourceType::Wikidata,
        name: "WorldBuilder",
        extensions: &["wld"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x45, 0x47, 0x49, 0x4E, 0x5F, 0x44, 0x45, 0x53, 0x49, 0x47, 0x4E,
                        0x3A, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
