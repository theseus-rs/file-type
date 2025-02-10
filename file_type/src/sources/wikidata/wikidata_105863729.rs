use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863729: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_729,
        source_type: SourceType::Wikidata,
        name: "PerFORM Messages",
        extensions: &["hlp", "msg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x65, 0x72, 0x46, 0x4F, 0x52, 0x4D, 0x20, 0x4D, 0x45, 0x53, 0x53,
                        0x41, 0x47, 0x45, 0x53, 0x20, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
