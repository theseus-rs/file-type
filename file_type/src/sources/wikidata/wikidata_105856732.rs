use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856732: FileFormat = FileFormat {
    id: 105_856_732,
    puid: "wikidata/105856732",
    name: "VisualBoyAdvance UPS patch",
    extensions: &["ups"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x50, 0x53, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
