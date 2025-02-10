use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857267: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_267,
        source_type: SourceType::Wikidata,
        name: "Help Magician Project",
        extensions: &["hmp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x4D, 0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x46,
                        0x69, 0x6C, 0x65, 0x20, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
