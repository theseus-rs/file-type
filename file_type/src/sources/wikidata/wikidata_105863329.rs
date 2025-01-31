use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863329: FileFormat = FileFormat {
    id: 105_863_329,
    puid: "wikidata/105863329",
    name: "Cubic Tiny XM module",
    extensions: &["mxm"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x58, 0x4D, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
