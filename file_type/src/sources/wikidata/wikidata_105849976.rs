use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849976: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_976,
        source_type: SourceType::Wikidata,
        name: "SeeYou Airspace",
        extensions: &["cub"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xC2, 0x43, 0x55, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
