use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850598: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_598,
        source_type: SourceType::Wikidata,
        name: "Calamus Ruler List",
        extensions: &["crl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x4D, 0x43, 0x20, 0x43, 0x41, 0x4C, 0x41, 0x4D, 0x55, 0x53, 0x20,
                        0x20, 0x43, 0x52, 0x4C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
