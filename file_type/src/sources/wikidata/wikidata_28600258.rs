use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600258: FileFormat = FileFormat {
    id: 28_600_258,
    puid: "wikidata/28600258",
    name: "ATR",
    extensions: &["atr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x96, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
