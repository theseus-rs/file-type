use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854358: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_358,
        source_type: SourceType::Wikidata,
        name: "Arahne weave",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x42, 0x49, 0x4E, 0x44, 0x49, 0x4E, 0x47,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
