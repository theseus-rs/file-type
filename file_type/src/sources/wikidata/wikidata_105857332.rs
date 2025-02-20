use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857332: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_332,
        source_type: SourceType::Wikidata,
        name: "Cura quality settings",
        extensions: &["json"],
        media_types: &["text/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x67, 0x65, 0x6E, 0x65, 0x72, 0x61, 0x6C, 0x5D, 0x0A, 0x76, 0x65,
                        0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
