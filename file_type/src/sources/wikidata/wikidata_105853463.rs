use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853463: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_463,
        source_type: SourceType::Wikidata,
        name: "Oren-made scientific document",
        extensions: &["zgt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xED, 0xFF, 0xFE, 0x07])],
                },
            }],
        }],
        related_formats: &[],
    },
};
