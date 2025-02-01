use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const CUSTOM_1: FileFormat = FileFormat {
    id: 1,
    puid: "custom/1",
    name: "Apache Arrow",
    extensions: &["arrow"],
    media_types: &["application/vnd.apache.arrow.file"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x52, 0x52, 0x4F, 0x57, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
