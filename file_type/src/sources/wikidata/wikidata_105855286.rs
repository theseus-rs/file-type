use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855286: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_286,
        source_type: SourceType::Wikidata,
        name: "FIFA game serie Font",
        extensions: &["ffn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x4E, 0x54, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
