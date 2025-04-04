use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855663: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_663,
        source_type: SourceType::Wikidata,
        name: "OneNote table of contents",
        extensions: &["onetoc2"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xA1, 0x2F, 0xFF, 0x43, 0xD9, 0xEF, 0x76, 0x4C, 0x9E, 0xE2, 0x10, 0xEA,
                        0x57, 0x22, 0x76, 0x5F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
