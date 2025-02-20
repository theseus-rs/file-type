use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860227: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_227,
        source_type: SourceType::Wikidata,
        name: "RTE encoded file",
        extensions: &["rte"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x54, 0x45, 0x4E, 0x43, 0x21, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
