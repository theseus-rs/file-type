use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855133: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_133,
        source_type: SourceType::Wikidata,
        name: "Vuforia QCAR Feature",
        extensions: &["feat"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x51, 0x43, 0x5F, 0x46, 0x45, 0x41, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
