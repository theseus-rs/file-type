use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856382: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_382,
        source_type: SourceType::Wikidata,
        name: "DeskMate document",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0D, 0x44, 0x4F, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
