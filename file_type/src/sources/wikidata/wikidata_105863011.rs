use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863011: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_011,
        source_type: SourceType::Wikidata,
        name: "Extra Simple Music module",
        extensions: &["xsm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6F, 0x66, 0x54, 0x41, 0x5A, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
