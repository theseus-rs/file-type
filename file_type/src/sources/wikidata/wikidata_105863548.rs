use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863548: FileFormat = FileFormat {
    id: 105_863_548,
    puid: "wikidata/105863548",
    name: "Alias|Wavefront material",
    extensions: &["mtl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0x65, 0x77, 0x6D, 0x74, 0x6C, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
