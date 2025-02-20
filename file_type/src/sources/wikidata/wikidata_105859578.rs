use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859578: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_578,
        source_type: SourceType::Wikidata,
        name: "Cargo! game data archive",
        extensions: &["vfs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x50, 0x33, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
