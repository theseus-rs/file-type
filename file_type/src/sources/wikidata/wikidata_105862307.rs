use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862307: FileFormat = FileFormat {
    id: 105_862_307,
    puid: "wikidata/105862307",
    name: "Melody Maker song (v3.x)",
    extensions: &["mm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEC, 0x4D, 0x4D, 0x76, 0x33, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
