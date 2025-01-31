use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206036: FileFormat = FileFormat {
    id: 28_206_036,
    puid: "wikidata/28206036",
    name: "Enhanced Simplex",
    extensions: &["esm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x4D, 0x53, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
