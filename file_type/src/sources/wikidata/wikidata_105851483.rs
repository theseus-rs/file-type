use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851483: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_483,
        source_type: SourceType::Wikidata,
        name: "Android Trash storage metadata",
        extensions: &["trashinfo"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x54, 0x72, 0x61, 0x73, 0x68, 0x20, 0x49, 0x6E, 0x66, 0x6F, 0x5D,
                        0x0A, 0x50, 0x61, 0x74, 0x68, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
