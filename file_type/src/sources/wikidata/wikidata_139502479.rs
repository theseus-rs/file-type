use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_139502479: FileType = FileType {
    file_format: &FileFormat {
        id: 139_502_479,
        source_type: SourceType::Wikidata,
        name: "Andrew Toolkit raster image",
        extensions: &["raster"],
        media_types: &["application/andrew-inset"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5C, 0x62, 0x65, 0x67, 0x69, 0x6E, 0x64, 0x61, 0x74, 0x61, 0x7B, 0x72,
                        0x61, 0x73, 0x74, 0x65, 0x72, 0x2C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
