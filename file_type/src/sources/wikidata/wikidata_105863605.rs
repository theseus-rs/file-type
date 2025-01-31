use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863605: FileFormat = FileFormat {
    id: 105_863_605,
    puid: "wikidata/105863605",
    name: "Luigi's Mansion 3D model",
    extensions: &["mdl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x04, 0xB4, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
