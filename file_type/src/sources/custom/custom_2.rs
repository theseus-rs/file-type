use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const CUSTOM_2: FileFormat = FileFormat {
    id: 2,
    puid: "custom/2",
    name: "Apache Avro",
    extensions: &["avro"],
    media_types: &["application/vnd.apache.avro.file"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x62, 0x6A, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
