use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_76142694: FileType = FileType {
    file_format: &FileFormat {
        id: 76_142_694,
        source_type: SourceType::Wikidata,
        name: "VisualBasic Project",
        extensions: &["vbp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x79, 0x70, 0x65, 0x3D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
