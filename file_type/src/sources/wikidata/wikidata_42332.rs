use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_42332: FileType = FileType {
    file_format: &FileFormat {
        id: 42_332,
        source_type: SourceType::Wikidata,
        name: "PDF",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x50, 0x44, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
