use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29905362: FileType = FileType {
    file_format: &FileFormat {
        id: 29_905_362,
        source_type: SourceType::Wikidata,
        name: "sK1",
        extensions: &["sk1"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x23, 0x73, 0x4B, 0x31, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
