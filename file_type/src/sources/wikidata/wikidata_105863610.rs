use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863610: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_610,
        source_type: SourceType::Wikidata,
        name: "Martin Fernandez Ad Lib module",
        extensions: &["adlib"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x44, 0x4C, 0x49, 0x42, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
