use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854096: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_096,
        source_type: SourceType::Wikidata,
        name: "ARQ archive",
        extensions: &["arq"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x67, 0x57, 0x04, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
