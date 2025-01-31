use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206946: FileFormat = FileFormat {
    id: 28_206_946,
    puid: "wikidata/28206946",
    name: "PhotoChrome",
    extensions: &["pcs"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x40, 0x00, 0xC8])],
            },
        }],
    }],
    related_formats: &[],
};
