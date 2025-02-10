use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854356: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_356,
        source_type: SourceType::Wikidata,
        name: "X-Plane Airfoils",
        extensions: &["afl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
