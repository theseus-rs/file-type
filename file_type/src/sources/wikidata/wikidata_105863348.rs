use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863348: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_348,
        source_type: SourceType::Wikidata,
        name: "MediaPoint script",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x43, 0x52, 0x49, 0x50, 0x54, 0x54, 0x41, 0x4C, 0x4B, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
