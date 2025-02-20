use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857602: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_602,
        source_type: SourceType::Wikidata,
        name: "G64 textual representation disk image",
        extensions: &["txt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6E, 0x6F, 0x2D, 0x74, 0x72, 0x61, 0x63, 0x6B, 0x73, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
