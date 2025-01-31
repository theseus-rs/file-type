use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855599: FileFormat = FileFormat {
    id: 105_855_599,
    puid: "wikidata/105855599",
    name: "Borland Overlay",
    extensions: &["ovr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x42, 0x4F, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
