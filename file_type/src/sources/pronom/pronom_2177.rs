use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2177: FileType = FileType {
    file_format: &FileFormat {
        id: 2_177,
        source_type: SourceType::Pronom,
        name: "Softdisk Text Compressor",
        extensions: &["ctx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x03, 0x43, 0x54, 0x30, 0x30, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
