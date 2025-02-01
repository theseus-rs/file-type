use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863896: FileFormat = FileFormat {
    id: 105_863_896,
    puid: "wikidata/105863896",
    name: "thinEdge model",
    extensions: &["m15"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x27, 0x74, 0x68, 0x69, 0x6E, 0x45, 0x64, 0x67, 0x65, 0x20, 0x64, 0x61, 0x74,
                    0x61, 0x66, 0x69, 0x6C, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
