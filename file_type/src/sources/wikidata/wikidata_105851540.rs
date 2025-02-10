use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851540: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_540,
        source_type: SourceType::Wikidata,
        name: "Adobe Tagged Text (ASCII)",
        extensions: &["txt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x41, 0x53, 0x43, 0x49, 0x49, 0x2D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
