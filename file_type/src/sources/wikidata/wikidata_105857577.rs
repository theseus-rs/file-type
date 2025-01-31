use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857577: FileFormat = FileFormat {
    id: 105_857_577,
    puid: "wikidata/105857577",
    name: "Actor Image snapshot (v4.1)",
    extensions: &["ima"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x10, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
