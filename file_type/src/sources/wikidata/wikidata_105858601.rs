use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858601: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_601,
        source_type: SourceType::Wikidata,
        name: "AliceSoft PMS bitmap",
        extensions: &["pms"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4D, 0x01, 0x00, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
