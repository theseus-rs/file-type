use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853452: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_452,
        source_type: SourceType::Wikidata,
        name: "Zing! directory tree",
        extensions: &["zingtree"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2A, 0x30, 0x30, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
