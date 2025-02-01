use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853927: FileFormat = FileFormat {
    id: 105_853_927,
    puid: "wikidata/105853927",
    name: "ArcView Legend",
    extensions: &["avl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2F, 0x33, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
