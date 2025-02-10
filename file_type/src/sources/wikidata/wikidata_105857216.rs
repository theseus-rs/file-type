use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857216: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_216,
        source_type: SourceType::Wikidata,
        name: "HiP compressed file",
        extensions: &["hip"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x69, 0x50, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
