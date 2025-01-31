use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864266: FileFormat = FileFormat {
    id: 105_864_266,
    puid: "wikidata/105864266",
    name: "Rumble Roses game data archive",
    extensions: &["pac"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x50, 0x41, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
