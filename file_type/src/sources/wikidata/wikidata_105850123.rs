use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850123: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_123,
        source_type: SourceType::Wikidata,
        name: "Windows codepage translator",
        extensions: &["cpx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x20, 0x4C, 0x61, 0x74,
                        0x69, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
