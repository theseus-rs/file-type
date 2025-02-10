use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858721: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_721,
        source_type: SourceType::Wikidata,
        name: "Encrypted JPEG bitmap",
        extensions: &["ejpg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x4A, 0x50, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
