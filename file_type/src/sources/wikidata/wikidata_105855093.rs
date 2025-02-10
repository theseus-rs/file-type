use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855093: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_093,
        source_type: SourceType::Wikidata,
        name: "DLC - DIGILINEAR compressed archive",
        extensions: &["dlc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x4C, 0x43, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
