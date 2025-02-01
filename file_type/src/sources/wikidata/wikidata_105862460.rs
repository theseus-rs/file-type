use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862460: FileFormat = FileFormat {
    id: 105_862_460,
    puid: "wikidata/105862460",
    name: "Custom Maid 3D 2 Model",
    extensions: &["model"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0A, 0x43, 0x4D, 0x33, 0x44, 0x32, 0x5F, 0x4D, 0x45, 0x53, 0x48,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
