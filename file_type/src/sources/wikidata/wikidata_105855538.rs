use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855538: FileFormat = FileFormat {
    id: 105_855_538,
    puid: "wikidata/105855538",
    name: "OziExplorer Map",
    extensions: &["ozfx3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x80, 0x77])],
            },
        }],
    }],
    related_formats: &[],
};
