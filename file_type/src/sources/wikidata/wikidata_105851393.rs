use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851393: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_393,
        source_type: SourceType::Wikidata,
        name: "TexMod package File",
        extensions: &["tpf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xF4, 0x74, 0xA7, 0x3B, 0xB0, 0x3F, 0xAD, 0x3F, 0xAC, 0x3F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
