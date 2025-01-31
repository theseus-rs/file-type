use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851947: FileFormat = FileFormat {
    id: 105_851_947,
    puid: "wikidata/105851947",
    name: "Mortal Kombat serie game data archive",
    extensions: &["ssf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x20, 0x43, 0x45, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
