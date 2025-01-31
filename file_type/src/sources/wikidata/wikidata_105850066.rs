use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850066: FileFormat = FileFormat {
    id: 105_850_066,
    puid: "wikidata/105850066",
    name: "Cube LUT format",
    extensions: &["cube"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x49, 0x54, 0x4C, 0x45, 0x20, 0x22])],
            },
        }],
    }],
    related_formats: &[],
};
