use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3566973: FileFormat = FileFormat {
    id: 3_566_973,
    puid: "wikidata/3566973",
    name: "WebVTT",
    extensions: &["vtt"],
    media_types: &["text/vtt"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x45, 0x42, 0x56, 0x54, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
