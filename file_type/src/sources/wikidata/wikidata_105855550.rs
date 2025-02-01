use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855550: FileFormat = FileFormat {
    id: 105_855_550,
    puid: "wikidata/105855550",
    name: "AWE Games game data container",
    extensions: &["omt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x30, 0x4D, 0x46, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
