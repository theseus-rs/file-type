use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862509: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_509,
        source_type: SourceType::Wikidata,
        name: "Max Patch",
        extensions: &["maxpat"],
        media_types: &["text/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
