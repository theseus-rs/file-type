use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862952: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_952,
        source_type: SourceType::Wikidata,
        name: "troff markup",
        extensions: &["me"],
        media_types: &["text/troff"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x54, 0x48, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
