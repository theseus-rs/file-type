use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855598: FileFormat = FileFormat {
    id: 105_855_598,
    puid: "wikidata/105855598",
    name: "Oberon/F Symbol File",
    extensions: &["osf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x53, 0x4F, 0x6F])],
            },
        }],
    }],
    related_formats: &[],
};
