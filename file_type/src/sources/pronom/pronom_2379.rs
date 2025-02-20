use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2379: FileType = FileType {
    file_format: &FileFormat {
        id: 2_379,
        source_type: SourceType::Pronom,
        name: "DNA Sequence Chromatogram File",
        extensions: &["scf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x73, 0x63, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
