use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2830: FileType = FileType {
    file_format: &FileFormat {
        id: 2_830,
        source_type: SourceType::Pronom,
        name: "Papyrus Document",
        extensions: &["pap", "pav", "pbf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x41, 0x50, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
