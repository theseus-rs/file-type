use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854659: FileFormat = FileFormat {
    id: 105_854_659,
    puid: "wikidata/105854659",
    name: "Caddie Tool Sets",
    extensions: &["atc"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x54, 0x6F, 0x6F, 0x6C, 0x53, 0x65, 0x74, 0x73, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
