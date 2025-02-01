use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866020: FileFormat = FileFormat {
    id: 105_866_020,
    puid: "wikidata/105866020",
    name: "Playstation Portable Texture",
    extensions: &["ppt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x70, 0x70, 0x74])],
            },
        }],
    }],
    related_formats: &[],
};
