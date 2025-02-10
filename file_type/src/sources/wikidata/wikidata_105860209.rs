use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860209: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_209,
        source_type: SourceType::Wikidata,
        name: "AskEnv Requester definition",
        extensions: &["req"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x49, 0x4E, 0x44, 0x4F, 0x57, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
