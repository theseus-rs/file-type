use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857556: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_556,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine spell (v1)",
        extensions: &["spl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x50, 0x4C, 0x20, 0x56, 0x31, 0x20, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
