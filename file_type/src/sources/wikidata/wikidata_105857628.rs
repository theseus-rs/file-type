use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857628: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_628,
        source_type: SourceType::Wikidata,
        name: "Virtual ][ disk image",
        extensions: &["v2d"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x35, 0x4E, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
