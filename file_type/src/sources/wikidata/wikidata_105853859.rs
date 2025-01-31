use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853859: FileFormat = FileFormat {
    id: 105_853_859,
    puid: "wikidata/105853859",
    name: "ARX compressed archive",
    extensions: &["arx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2D, 0x6C, 0x68, 0x31, 0x2D, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
