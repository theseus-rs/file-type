use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863234: FileFormat = FileFormat {
    id: 105_863_234,
    puid: "wikidata/105863234",
    name: "Magnetic Sound",
    extensions: &["snd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x61, 0x53, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
