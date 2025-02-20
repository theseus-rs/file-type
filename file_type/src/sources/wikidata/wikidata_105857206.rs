use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857206: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_206,
        source_type: SourceType::Wikidata,
        name: "High Voltage SID Collection update info",
        extensions: &["hvs"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x20, 0x48, 0x69, 0x67, 0x68, 0x20, 0x56, 0x6F, 0x6C, 0x74, 0x61,
                        0x67, 0x65, 0x20, 0x53, 0x49, 0x44, 0x20, 0x43, 0x6F, 0x6C, 0x6C, 0x65,
                        0x63, 0x74, 0x69, 0x6F, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
