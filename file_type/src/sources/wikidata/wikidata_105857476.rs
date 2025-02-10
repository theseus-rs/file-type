use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857476: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_476,
        source_type: SourceType::Wikidata,
        name: "Generic 3D Drafting drawing",
        extensions: &["3dd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x00, 0x01, 0x20, 0x33, 0x44, 0x20, 0x50, 0x4C, 0x41, 0x4E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
