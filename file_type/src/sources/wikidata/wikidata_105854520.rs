use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854520: FileFormat = FileFormat {
    id: 105_854_520,
    puid: "wikidata/105854520",
    name: "CPIO archive (byte swapped binary)",
    extensions: &["cpio"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x71, 0xC7])],
            },
        }],
    }],
    related_formats: &[],
};
