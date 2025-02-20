use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863395: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_395,
        source_type: SourceType::Wikidata,
        name: "Standard 4-channel Amiga module",
        extensions: &["mod"],
        media_types: &["audio/mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x2E, 0x4B, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
