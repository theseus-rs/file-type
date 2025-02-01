use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863973: FileFormat = FileFormat {
    id: 105_863_973,
    puid: "wikidata/105863973",
    name: "Macro Express Macro",
    extensions: &["mex"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x45, 0x00, 0x00, 0xBB, 0x0B, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
