use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854194: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_194,
        source_type: SourceType::Wikidata,
        name: "DEC-WSE Object File Format (text, start with LF)",
        extensions: &["aoff"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
