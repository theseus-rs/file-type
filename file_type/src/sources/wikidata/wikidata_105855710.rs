use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855710: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_710,
        source_type: SourceType::Wikidata,
        name: "ObjectVision Datafile",
        extensions: &["ovd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x19, 0xA4, 0x14, 0x00, 0x24, 0x00,
                        0x02, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
