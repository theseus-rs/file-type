use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863264: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_264,
        source_type: SourceType::Wikidata,
        name: "Metalink (v3/old)",
        extensions: &["metalink"],
        media_types: &["application/metalink+xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
