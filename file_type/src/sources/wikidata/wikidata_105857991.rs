use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857991: FileFormat = FileFormat {
    id: 105_857_991,
    puid: "wikidata/105857991",
    name: "Sonic Global Image",
    extensions: &["gi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xDA, 0xDA, 0xFE, 0xFE])],
            },
        }],
    }],
    related_formats: &[],
};
