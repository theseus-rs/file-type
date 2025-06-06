use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865109: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_109,
        source_type: SourceType::Wikidata,
        name: "DeLorme map data",
        extensions: &["pm0"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x4D, 0x61, 0x70, 0x58, 0x4D, 0x61, 0x70,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
