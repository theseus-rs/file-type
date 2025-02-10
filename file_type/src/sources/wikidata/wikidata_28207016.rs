use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28207016: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_016,
        source_type: SourceType::Wikidata,
        name: "Pixar picture",
        extensions: &["pic", "picio", "pixar", "pxr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x80, 0xE8, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
