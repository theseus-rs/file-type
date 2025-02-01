use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850983: FileFormat = FileFormat {
    id: 105_850_983,
    puid: "wikidata/105850983",
    name: "FOnline Engine Tile set",
    extensions: &["til"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x74, 0x69, 0x6C, 0x65, 0x3E])],
            },
        }],
    }],
    related_formats: &[],
};
