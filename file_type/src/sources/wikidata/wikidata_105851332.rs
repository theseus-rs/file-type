use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851332: FileFormat = FileFormat {
    id: 105_851_332,
    puid: "wikidata/105851332",
    name: "U.S. Navy Fighters Theater game data",
    extensions: &["t2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x49, 0x54, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
