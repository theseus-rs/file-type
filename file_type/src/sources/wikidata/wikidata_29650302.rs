use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650302: FileFormat = FileFormat {
    id: 29_650_302,
    puid: "wikidata/29650302",
    name: "Perfect ZX Tape",
    extensions: &["pzx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x5A, 0x58, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
