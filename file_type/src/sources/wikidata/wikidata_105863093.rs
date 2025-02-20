use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863093: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_093,
        source_type: SourceType::Wikidata,
        name: "Palladix Ad Lib module",
        extensions: &["plx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4C, 0x58, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
