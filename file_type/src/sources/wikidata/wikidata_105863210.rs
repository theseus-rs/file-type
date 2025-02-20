use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863210: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_210,
        source_type: SourceType::Wikidata,
        name: "The Player 4.1a module",
        extensions: &["p41"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x34, 0x31, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
