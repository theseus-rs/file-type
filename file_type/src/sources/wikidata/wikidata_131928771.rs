use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131928771: FileFormat = FileFormat {
    id: 131_928_771,
    puid: "wikidata/131928771",
    name: "ConceptDraw Document",
    extensions: &["cdd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4F, 0x4E, 0x43, 0x45, 0x50, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
