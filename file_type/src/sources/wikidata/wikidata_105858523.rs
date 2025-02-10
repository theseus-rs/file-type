use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858523: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_523,
        source_type: SourceType::Wikidata,
        name: "EZ-Art Professional bitmap",
        extensions: &["eza"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x5A, 0x00, 0xC8])],
                },
            }],
        }],
        related_formats: &[],
    },
};
