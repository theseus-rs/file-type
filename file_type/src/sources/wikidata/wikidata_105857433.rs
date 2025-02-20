use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857433: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_433,
        source_type: SourceType::Wikidata,
        name: "Atlantis Evolution game data archive",
        extensions: &["jp6"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x50, 0x36, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
