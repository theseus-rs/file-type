use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850431: FileFormat = FileFormat {
    id: 105_850_431,
    puid: "wikidata/105850431",
    name: "Royal Heroes game data archive",
    extensions: &["car"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0x61, 0x63, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
