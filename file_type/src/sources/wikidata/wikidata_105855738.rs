use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855738: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_738,
        source_type: SourceType::Wikidata,
        name: "Meridian Driver",
        extensions: &["drv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x25, 0x4D, 0x45, 0x52, 0x5F, 0x44, 0x52, 0x56, 0x25, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
