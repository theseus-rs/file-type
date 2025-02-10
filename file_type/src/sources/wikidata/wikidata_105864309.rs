use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864309: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_309,
        source_type: SourceType::Wikidata,
        name: "Philips Drum System data (generic)",
        extensions: &["drm", "dsq"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x68, 0x69, 0x6C, 0x69, 0x70, 0x73, 0x20, 0x44, 0x72, 0x75, 0x6D,
                        0x20, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
