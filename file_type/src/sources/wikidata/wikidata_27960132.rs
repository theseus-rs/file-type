use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27960132: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_132,
        source_type: SourceType::Wikidata,
        name: "Bonk",
        extensions: &["bonk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x42, 0x4F, 0x4E, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
