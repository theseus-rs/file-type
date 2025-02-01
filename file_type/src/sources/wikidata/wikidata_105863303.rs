use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863303: FileFormat = FileFormat {
    id: 105_863_303,
    puid: "wikidata/105863303",
    name: "SawTeeth module",
    extensions: &["st"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x57, 0x54, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
