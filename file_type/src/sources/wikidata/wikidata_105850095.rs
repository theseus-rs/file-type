use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850095: FileFormat = FileFormat {
    id: 105_850_095,
    puid: "wikidata/105850095",
    name: "Ventura Publisher Chapter",
    extensions: &["chp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x44, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
