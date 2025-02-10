use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855594: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_594,
        source_type: SourceType::Wikidata,
        name: "Mozart functor",
        extensions: &["ozf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x02, 0x02, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
