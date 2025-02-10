use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861495: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_495,
        source_type: SourceType::Wikidata,
        name: "Seal Link",
        extensions: &["ln"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x64, 0x65, 0x66, 0x69, 0x6E, 0x69, 0x74, 0x69, 0x6F, 0x6E, 0x5D,
                        0x0D, 0x0A, 0x6C, 0x69, 0x6E, 0x6B, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
