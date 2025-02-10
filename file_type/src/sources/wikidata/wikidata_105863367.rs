use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863367: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_367,
        source_type: SourceType::Wikidata,
        name: "Earth And Beyond game data archive",
        extensions: &["mix"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x49, 0x58, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
