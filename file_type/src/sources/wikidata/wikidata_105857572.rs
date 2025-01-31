use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857572: FileFormat = FileFormat {
    id: 105_857_572,
    puid: "wikidata/105857572",
    name: "Infinity Engine creature (v1.2)",
    extensions: &["cre"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x52, 0x45, 0x20, 0x56, 0x31, 0x2E, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
