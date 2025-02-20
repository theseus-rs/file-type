use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863924: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_924,
        source_type: SourceType::Wikidata,
        name: "MPAK game data archive",
        extensions: &["mpak"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x50, 0x41, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
