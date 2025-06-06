use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860037: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_037,
        source_type: SourceType::Wikidata,
        name: "MSHeli Vbar data",
        extensions: &["vbr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x52, 0x45, 0x47, 0x49, 0x53, 0x54, 0x45, 0x52, 0x3E, 0x0D, 0x0A,
                        0x20, 0x20, 0x20, 0x20, 0x3C, 0x56, 0x41, 0x4C, 0x55, 0x45, 0x20, 0x52,
                        0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x3D, 0x22, 0x30, 0x22, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
