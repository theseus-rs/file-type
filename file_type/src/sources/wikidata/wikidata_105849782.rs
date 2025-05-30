use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849782: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_782,
        source_type: SourceType::Wikidata,
        name: "Circuit Diagram Document",
        extensions: &["cddx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x28, 0x41, 0x67, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
