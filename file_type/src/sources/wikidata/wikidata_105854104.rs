use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854104: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_104,
        source_type: SourceType::Wikidata,
        name: "Aley's Module v1.1-1.2",
        extensions: &["alm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x6C, 0x65, 0x79, 0x4D, 0x6F, 0x64])],
                },
            }],
        }],
        related_formats: &[],
    },
};
