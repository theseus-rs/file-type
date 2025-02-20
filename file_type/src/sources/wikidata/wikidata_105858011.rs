use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858011: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_011,
        source_type: SourceType::Wikidata,
        name: "Apple II ProDOS disk image",
        extensions: &["dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x38, 0xB0, 0x03, 0x4C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
