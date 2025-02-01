use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857725: FileFormat = FileFormat {
    id: 105_857_725,
    puid: "wikidata/105857725",
    name: "Infinity Engine Store (v1.1)",
    extensions: &["sto"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x54, 0x4F, 0x52, 0x56, 0x31, 0x2E, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
