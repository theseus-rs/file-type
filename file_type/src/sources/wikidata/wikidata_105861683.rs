use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861683: FileFormat = FileFormat {
    id: 105_861_683,
    puid: "wikidata/105861683",
    name: "Mophun game",
    extensions: &["mpn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x4D, 0x47, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
