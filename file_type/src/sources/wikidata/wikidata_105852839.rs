use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852839: FileFormat = FileFormat {
    id: 105_852_839,
    puid: "wikidata/105852839",
    name: "You Don't Know Jack game data archive",
    extensions: &["srf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x72, 0x66, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
