use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863192: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_192,
        source_type: SourceType::Wikidata,
        name: "Unity asset bundles Manifest",
        extensions: &["manifest"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x61, 0x6E, 0x69, 0x66, 0x65, 0x73, 0x74, 0x46, 0x69, 0x6C, 0x65,
                        0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
