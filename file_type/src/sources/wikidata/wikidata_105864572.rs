use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864572: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_572,
        source_type: SourceType::Wikidata,
        name: "Dynamix Palette",
        extensions: &["pal"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x41, 0x4C, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
