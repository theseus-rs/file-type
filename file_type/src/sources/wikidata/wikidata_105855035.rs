use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855035: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_035,
        source_type: SourceType::Wikidata,
        name: "Windows Policy Administrative Template",
        extensions: &["adm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4C, 0x41, 0x53, 0x53, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
