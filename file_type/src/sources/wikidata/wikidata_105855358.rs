use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855358: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_358,
        source_type: SourceType::Wikidata,
        name: "Bleach game data archive",
        extensions: &["fpk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x78, 0x4A, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
