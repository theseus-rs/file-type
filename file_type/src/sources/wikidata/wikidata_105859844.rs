use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859844: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_844,
        source_type: SourceType::Wikidata,
        name: "VMware configuration (Unix like ver.)",
        extensions: &["vmx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x21, 0x2F, 0x75, 0x73, 0x72, 0x2F, 0x62, 0x69, 0x6E, 0x2F, 0x76,
                        0x6D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
