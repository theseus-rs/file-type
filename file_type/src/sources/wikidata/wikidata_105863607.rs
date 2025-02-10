use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863607: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_607,
        source_type: SourceType::Wikidata,
        name: "Metalink (v4)",
        extensions: &["meta4"],
        media_types: &["application/metalink4+xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
