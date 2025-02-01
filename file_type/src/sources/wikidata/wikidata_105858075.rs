use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858075: FileFormat = FileFormat {
    id: 105_858_075,
    puid: "wikidata/105858075",
    name: "+3DOS / CP/M disk image",
    extensions: &["p3d"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x33, 0x44, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
