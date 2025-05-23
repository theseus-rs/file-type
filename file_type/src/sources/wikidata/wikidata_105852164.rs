use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852164: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_164,
        source_type: SourceType::Wikidata,
        name: "PageRender3D Setup",
        extensions: &["setup"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x73, 0x65, 0x74, 0x75, 0x70, 0x20,
                        0x20, 0x20, 0x20, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x6E, 0x61, 0x6D, 0x65,
                        0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
