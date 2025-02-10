use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_21039273: FileType = FileType {
    file_format: &FileFormat {
        id: 21_039_273,
        source_type: SourceType::Wikidata,
        name: "ASYLUM Music Format",
        extensions: &["amf"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x53, 0x59, 0x4C, 0x55, 0x4D, 0x20, 0x4D, 0x75, 0x73, 0x69, 0x63,
                        0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20, 0x56, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
