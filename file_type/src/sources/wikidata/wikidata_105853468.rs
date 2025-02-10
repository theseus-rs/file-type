use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853468: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_468,
        source_type: SourceType::Wikidata,
        name: "ZMA impedance response data",
        extensions: &["zma"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x7A, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x4F, 0x68, 0x6D,
                        0x73, 0x20, 0x20, 0x20, 0x20, 0x44, 0x65, 0x67,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
