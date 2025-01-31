use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850120: FileFormat = FileFormat {
    id: 105_850_120,
    puid: "wikidata/105850120",
    name: "CorelDRAW drawing (zipped)",
    extensions: &["cdr"],
    media_types: &["application/x-coreldraw"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
