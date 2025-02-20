use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853506: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_506,
        source_type: SourceType::Wikidata,
        name: "ZX-Edit document block",
        extensions: &["zed"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x58, 0x2D, 0x45, 0x64, 0x69, 0x74, 0x20, 0x64, 0x6F, 0x63, 0x75,
                        0x6D, 0x65, 0x6E, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
