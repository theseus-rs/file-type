use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862968: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_968,
        source_type: SourceType::Wikidata,
        name: "MOZART Percussion map",
        extensions: &["mzp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x45, 0x52, 0x43, 0x55, 0x53, 0x53, 0x49, 0x4F, 0x4E, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
