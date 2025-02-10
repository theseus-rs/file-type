use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857384: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_384,
        source_type: SourceType::Wikidata,
        name: "Labeler (v2.0) / Labels Unlimited (v1.0) Job",
        extensions: &["job"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x61, 0x62, 0x65, 0x6C, 0x65, 0x72, 0x20, 0x56, 0x32, 0x2E, 0x30,
                        0x20, 0x4A, 0x4F, 0x42, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
