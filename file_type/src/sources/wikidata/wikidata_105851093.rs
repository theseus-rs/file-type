use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851093: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_093,
        source_type: SourceType::Wikidata,
        name: "TurboCAD drawing (v3 Win)",
        extensions: &["tcw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
