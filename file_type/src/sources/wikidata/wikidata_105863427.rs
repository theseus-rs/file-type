use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863427: FileFormat = FileFormat {
    id: 105_863_427,
    puid: "wikidata/105863427",
    name: "PlayStation RSD animation (v3.0)",
    extensions: &["mot"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x40, 0x4D, 0x4F, 0x54, 0x39, 0x37, 0x30, 0x34, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
