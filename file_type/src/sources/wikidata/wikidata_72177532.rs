use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72177532: FileType = FileType {
    file_format: &FileFormat {
        id: 72_177_532,
        source_type: SourceType::Wikidata,
        name: "kRAW Audio Stream",
        extensions: &["kraw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6B, 0x52, 0x41, 0x57])],
                },
            }],
        }],
        related_formats: &[],
    },
};
