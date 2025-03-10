use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855231: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_231,
        source_type: SourceType::Wikidata,
        name: "Design Simulations physics data (generic)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x7A, 0x7A, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
