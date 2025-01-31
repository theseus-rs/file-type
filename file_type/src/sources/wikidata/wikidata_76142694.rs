use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76142694: FileFormat = FileFormat {
    id: 76_142_694,
    puid: "wikidata/76142694",
    name: "VisualBasic Project",
    extensions: &["vbp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x79, 0x70, 0x65, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
