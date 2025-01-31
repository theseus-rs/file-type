use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858521: FileFormat = FileFormat {
    id: 105_858_521,
    puid: "wikidata/105858521",
    name: "Colin McRae Rally 2 game data archive",
    extensions: &["bfl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4D, 0x50, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
