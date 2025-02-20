use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850233: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_233,
        source_type: SourceType::Wikidata,
        name: "The Virtual ColecoVision Save Game",
        extensions: &["csg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x53, 0x47, 0x0A, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
