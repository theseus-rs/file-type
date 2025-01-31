use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856374: FileFormat = FileFormat {
    id: 105_856_374,
    puid: "wikidata/105856374",
    name: "XL/ST link / XLDJ Disk Image",
    extensions: &["di"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x49, 0x20, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
