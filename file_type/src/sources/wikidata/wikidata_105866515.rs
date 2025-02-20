use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866515: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_515,
        source_type: SourceType::Wikidata,
        name: "CyberLink PowerProducer Project",
        extensions: &["ppp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x50, 0x50, 0x5F, 0x30, 0x30, 0x30, 0x31, 0x2E, 0x30, 0x30, 0x30,
                        0x44, 0x42, 0x47, 0x4E, 0x56, 0x49, 0x44, 0x45, 0x4F, 0x5F, 0x42, 0x55,
                        0x52, 0x4E, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
