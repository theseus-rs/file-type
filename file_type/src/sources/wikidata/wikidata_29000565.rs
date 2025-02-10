use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_29000565: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_565,
        source_type: SourceType::Wikidata,
        name: "SuperCard Pro dump",
        extensions: &["scp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x43, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
