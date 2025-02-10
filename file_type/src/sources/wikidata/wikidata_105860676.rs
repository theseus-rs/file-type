use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860676: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_676,
        source_type: SourceType::Wikidata,
        name: "Reaction Data file",
        extensions: &["rd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x24, 0x52, 0x44, 0x46, 0x49, 0x4C, 0x45, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
