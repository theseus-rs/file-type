use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1989: FileType = FileType {
    file_format: &FileFormat {
        id: 1_989,
        source_type: SourceType::Pronom,
        name: "Away3D Data Format",
        extensions: &["awd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x57, 0x44, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
