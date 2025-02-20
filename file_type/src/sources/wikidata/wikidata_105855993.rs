use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855993: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_993,
        source_type: SourceType::Wikidata,
        name: "iGO Digital Elevation Map",
        extensions: &["dem"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x45, 0x4D, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
