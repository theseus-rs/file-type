use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865454: FileFormat = FileFormat {
    id: 105_865_454,
    puid: "wikidata/105865454",
    name: "Pro Trekkr 2.0 module",
    extensions: &["ptk"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x52, 0x4F, 0x54, 0x52, 0x45, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
