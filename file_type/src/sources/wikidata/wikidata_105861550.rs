use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861550: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_550,
        source_type: SourceType::Wikidata,
        name: "DNA Sequence Alignment",
        extensions: &["lav"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x3A, 0x6C, 0x61, 0x76])],
                },
            }],
        }],
        related_formats: &[],
    },
};
