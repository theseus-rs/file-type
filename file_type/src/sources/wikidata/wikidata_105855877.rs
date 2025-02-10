use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855877: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_877,
        source_type: SourceType::Wikidata,
        name: "Talisman game data archive",
        extensions: &["dz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x54, 0x52, 0x5A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
