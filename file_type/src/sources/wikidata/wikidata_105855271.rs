use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855271: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_271,
        source_type: SourceType::Wikidata,
        name: "FireFly menu definition",
        extensions: &["txt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x46, 0x49, 0x52, 0x45, 0x46, 0x4C, 0x59, 0x5F, 0x54, 0x4F, 0x50,
                        0x4D, 0x45, 0x4E, 0x55, 0x23, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
