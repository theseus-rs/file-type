use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857457: FileFormat = FileFormat {
    id: 105_857_457,
    source_type: SourceType::Wikidata,
    name: "QuickDraw 3D Metafile (binary, LE)",
    extensions: &["3dmf"],
    media_types: &["x-world/x-3dmf"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x4D, 0x44, 0x33, 0x10, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
