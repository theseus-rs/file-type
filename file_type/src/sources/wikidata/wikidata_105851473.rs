use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851473: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_473,
        source_type: SourceType::Wikidata,
        name: "Q105851473",
        extensions: &["typelib"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x4F, 0x42, 0x4A, 0x0A, 0x4D, 0x45, 0x54, 0x41, 0x44, 0x41, 0x54,
                        0x41, 0x0D, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
