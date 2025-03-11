use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861044: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_044,
        source_type: SourceType::Wikidata,
        name: "Landscape Designer fractal data",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x72, 0x61, 0x63, 0x74, 0x61, 0x6C, 0x64, 0x61, 0x74, 0x61, 0x73,
                        0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
