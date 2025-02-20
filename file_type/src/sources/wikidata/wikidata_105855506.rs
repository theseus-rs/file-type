use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855506: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_506,
        source_type: SourceType::Wikidata,
        name: "The Fractal Mapper map",
        extensions: &["fmp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x42, 0x65, 0x67, 0x69, 0x6E, 0x5D, 0x0D, 0x0A, 0x44, 0x65, 0x66,
                        0x69, 0x6E, 0x69, 0x74, 0x69, 0x6F, 0x6E, 0x3A, 0x20, 0x54, 0x68, 0x65,
                        0x20, 0x46, 0x72, 0x61, 0x63, 0x74, 0x61, 0x6C, 0x20, 0x4D, 0x61, 0x70,
                        0x70, 0x65, 0x72, 0x20, 0x4D, 0x61, 0x70, 0x20, 0x46, 0x69, 0x6C, 0x65,
                        0x0D, 0x0A, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x3A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
