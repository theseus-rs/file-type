use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857455: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_455,
        source_type: SourceType::Wikidata,
        name: "3D Text Animator character",
        extensions: &["anim"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x66, 0x61, 0x63, 0x65, 0x74, 0x20, 0x62, 0x69, 0x6E, 0x61, 0x72, 0x79,
                        0x0A, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
