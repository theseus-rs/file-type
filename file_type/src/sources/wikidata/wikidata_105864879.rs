use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864879: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_879,
        source_type: SourceType::Wikidata,
        name: "HSQLDB configuration",
        extensions: &["properties"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x48, 0x53, 0x51, 0x4C, 0x20, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61,
                        0x73, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
