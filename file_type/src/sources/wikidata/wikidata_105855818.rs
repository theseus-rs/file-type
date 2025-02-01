use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855818: FileFormat = FileFormat {
    id: 105_855_818,
    puid: "wikidata/105855818",
    name: "Daintree SNA Capture File (v4)",
    extensions: &["dcf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x3D, 0x34,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
