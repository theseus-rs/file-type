use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854196: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_196,
        source_type: SourceType::Wikidata,
        name: "shar SHell self-extracting aRchive",
        extensions: &["sha", "shar", "shr"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
