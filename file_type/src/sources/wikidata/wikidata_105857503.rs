use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857503: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_503,
        source_type: SourceType::Wikidata,
        name: "QuickDraw 3D Metafile (text)",
        extensions: &["3dmf"],
        media_types: &["x-world/x-3dmf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x33, 0x44, 0x4D, 0x65, 0x74, 0x61, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x28,
                        0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
