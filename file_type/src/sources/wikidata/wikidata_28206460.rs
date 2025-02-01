use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206460: FileFormat = FileFormat {
    id: 28_206_460,
    puid: "wikidata/28206460",
    name: "KiSS color file",
    extensions: &["kcf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x69, 0x53, 0x53, 0x10])],
            },
        }],
    }],
    related_formats: &[],
};
