use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853323: FileFormat = FileFormat {
    id: 105_853_323,
    puid: "wikidata/105853323",
    name: "ASCD snapshot (unGZipped)",
    extensions: &["scs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x61, 0x6D, 0x53, 0x6E, 0x61, 0x70, 0x21, 0x41, 0x53, 0x43, 0x44,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
