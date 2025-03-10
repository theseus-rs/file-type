use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851401: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_401,
        source_type: SourceType::Wikidata,
        name: "Turbo Packer compressed data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x50, 0x57, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
