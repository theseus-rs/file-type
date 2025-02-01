use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850956: FileFormat = FileFormat {
    id: 105_850_956,
    puid: "wikidata/105850956",
    name: "Trackjoy GUS Tracker song",
    extensions: &["tjs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x52, 0x41, 0x43, 0x4B, 0x4A, 0x4F, 0x59, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
