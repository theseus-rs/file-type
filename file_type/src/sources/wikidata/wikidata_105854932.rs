use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854932: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_932,
        source_type: SourceType::Wikidata,
        name: "BRL-CAD Geometry (ASCII)",
        extensions: &["asc"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x74, 0x69, 0x74, 0x6C, 0x65, 0x20, 0x7B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
