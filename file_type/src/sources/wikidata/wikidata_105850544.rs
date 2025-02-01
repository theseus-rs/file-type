use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850544: FileFormat = FileFormat {
    id: 105_850_544,
    puid: "wikidata/105850544",
    name: "16bit DOS COM Mess encrypted (v1.x)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x55, 0x43, 0x4B, 0x59, 0x4F, 0x55, 0x1A, 0xFF, 0x5F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
