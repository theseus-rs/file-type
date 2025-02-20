use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860347: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_347,
        source_type: SourceType::Wikidata,
        name: "R documentation (with rem)",
        extensions: &["rd"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25])],
                },
            }],
        }],
        related_formats: &[],
    },
};
