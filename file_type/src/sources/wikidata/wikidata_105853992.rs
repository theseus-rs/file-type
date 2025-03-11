use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853992: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_992,
        source_type: SourceType::Wikidata,
        name: "AmigaKonto account",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x43, 0x43, 0x4F, 0x55, 0x4E, 0x54, 0x5F, 0x46, 0x49, 0x4C, 0x45,
                        0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
