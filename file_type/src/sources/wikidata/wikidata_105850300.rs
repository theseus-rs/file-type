use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850300: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_300,
        source_type: SourceType::Wikidata,
        name: "Calamus Font Data",
        extensions: &["cfn"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x41, 0x4C, 0x41, 0x4D, 0x55, 0x53, 0x43, 0x46, 0x4E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
