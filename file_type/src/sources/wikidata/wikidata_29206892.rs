use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_29206892: FileType = FileType {
    file_format: &FileFormat {
        id: 29_206_892,
        source_type: SourceType::Wikidata,
        name: "ICC profile, version 4.3.0.0",
        extensions: &["icc", "icm"],
        media_types: &["application/vnd.iccprofile"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x61, 0x63, 0x73, 0x70])],
                },
            }],
        }],
        related_formats: &[],
    },
};
