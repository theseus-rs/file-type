use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863739: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_739,
        source_type: SourceType::Wikidata,
        name: "Mall Tycoon game data archive",
        extensions: &["muk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x55, 0x4B, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x28, 0x43, 0x29,
                        0x20, 0x48, 0x6F, 0x6C, 0x69, 0x73, 0x74, 0x69, 0x63, 0x20, 0x44, 0x65,
                        0x73, 0x69, 0x67, 0x6E, 0x20, 0x32, 0x30, 0x30, 0x30, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
