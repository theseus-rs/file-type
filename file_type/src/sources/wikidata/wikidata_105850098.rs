use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850098: FileFormat = FileFormat {
    id: 105_850_098,
    puid: "wikidata/105850098",
    name: "Flexidump Custom printer driver",
    extensions: &["cus"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x43, 0x55, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
