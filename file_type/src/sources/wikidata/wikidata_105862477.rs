use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862477: FileFormat = FileFormat {
    id: 105_862_477,
    puid: "wikidata/105862477",
    name: "MDIFF patch (v1.x)",
    extensions: &["mdf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x44, 0x46, 0x00, 0x31, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
