use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860738: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_738,
        source_type: SourceType::Wikidata,
        name: "Revolution MetaCard stack (legacy)",
        extensions: &["mc", "rev"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x21, 0x2F, 0x62, 0x69, 0x6E, 0x2F, 0x73, 0x68, 0x0A, 0x23, 0x20,
                        0x4D, 0x65, 0x74, 0x61, 0x43, 0x61, 0x72, 0x64, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
