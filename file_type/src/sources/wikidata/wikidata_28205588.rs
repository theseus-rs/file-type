use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205588: FileFormat = FileFormat {
    id: 28_205_588,
    puid: "wikidata/28205588",
    name: "PaintShop Pro Browser Cache",
    extensions: &["jbf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x41, 0x53, 0x43, 0x20, 0x42, 0x52, 0x4F, 0x57, 0x53, 0x20, 0x46, 0x49,
                    0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
