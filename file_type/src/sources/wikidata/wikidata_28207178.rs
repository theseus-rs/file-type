use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28207178: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_178,
        source_type: SourceType::Wikidata,
        name: "Q0 Image Attributes",
        extensions: &["fal"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x5F, 0x41, 0x4C, 0x4C, 0x28, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
