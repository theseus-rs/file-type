use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860743: FileFormat = FileFormat {
    id: 105_860_743,
    puid: "wikidata/105860743",
    name: "RealMedia File container",
    extensions: &["rmf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x52, 0x4D, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
