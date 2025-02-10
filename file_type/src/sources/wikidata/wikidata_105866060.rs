use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866060: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_060,
        source_type: SourceType::Wikidata,
        name: "Protracker Studio 16 module/song",
        extensions: &["ps16", "psm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x53, 0x31, 0x36, 0xFE])],
                },
            }],
        }],
        related_formats: &[],
    },
};
