use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7096031: FileFormat = FileFormat {
    id: 7_096_031,
    puid: "wikidata/7096031",
    name: "Open Financial Connectivity",
    extensions: &["ofc"],
    media_types: &["text/ofc"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x4F, 0x46, 0x43, 0x3E])],
            },
        }],
    }],
    related_formats: &[],
};
