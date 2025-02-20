use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864657: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_657,
        source_type: SourceType::Wikidata,
        name: "Aegis Animator Polygon",
        extensions: &["poly"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2A, 0x61, 0x63, 0x74, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
