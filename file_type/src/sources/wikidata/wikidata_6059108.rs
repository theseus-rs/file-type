use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_6059108: FileType = FileType {
    file_format: &FileFormat {
        id: 6_059_108,
        source_type: SourceType::Wikidata,
        name: "Intuit Interchange Format",
        extensions: &["iif"],
        media_types: &["application/qbooks", "application/qbookspro", "text/iif"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
