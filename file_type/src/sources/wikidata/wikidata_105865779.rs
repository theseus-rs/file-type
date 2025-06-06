use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865779: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_779,
        source_type: SourceType::Wikidata,
        name: "Hugin Project",
        extensions: &["pto"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
