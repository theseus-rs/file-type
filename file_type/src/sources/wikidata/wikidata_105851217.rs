use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851217: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_217,
        source_type: SourceType::Wikidata,
        name: "Total Commander Tabs configuration",
        extensions: &["tab"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x74, 0x61, 0x62, 0x73, 0x5D,
                        0x0D, 0x0A, 0x30, 0x5F, 0x70, 0x61, 0x74, 0x68, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
