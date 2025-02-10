use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28207412: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_412,
        source_type: SourceType::Wikidata,
        name: "Valve Texture Format",
        extensions: &["vtf"],
        media_types: &["image/vnd.valve.source.texture"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x54, 0x46, 0x00, 0x07, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
