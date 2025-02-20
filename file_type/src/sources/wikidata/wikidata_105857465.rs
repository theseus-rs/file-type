use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857465: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_465,
        source_type: SourceType::Wikidata,
        name: "Cadent 3D Model",
        extensions: &["3dm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x61, 0x64, 0x65, 0x6E, 0x74, 0x20, 0x33, 0x44, 0x20, 0x4D, 0x6F,
                        0x64, 0x65, 0x6C, 0x20, 0x56,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
