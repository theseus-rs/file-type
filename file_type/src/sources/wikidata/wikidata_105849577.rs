use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849577: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_577,
        source_type: SourceType::Wikidata,
        name: "CableNut Custom Settings",
        extensions: &["ccs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x61, 0x62, 0x6C, 0x65, 0x4E, 0x75, 0x74, 0x20, 0x43, 0x75, 0x73,
                        0x74, 0x6F, 0x6D, 0x20, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6E, 0x67, 0x73,
                        0x20, 0x46, 0x69, 0x6C, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
