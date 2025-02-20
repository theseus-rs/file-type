use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851244: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_244,
        source_type: SourceType::Wikidata,
        name: "Borland Turbo Debugger session-state settings",
        extensions: &["tr2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x75, 0x72, 0x62, 0x6F, 0x20, 0x44, 0x65, 0x62, 0x75, 0x67, 0x67,
                        0x65, 0x72, 0x20, 0x72, 0x65, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x66,
                        0x69, 0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
