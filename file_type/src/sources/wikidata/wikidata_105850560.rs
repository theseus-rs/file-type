use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850560: FileFormat = FileFormat {
    id: 105_850_560,
    puid: "wikidata/105850560",
    name: "Class Diagram (Unicode)",
    extensions: &["cd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x3F, 0x78, 0x6D, 0x6C])],
            },
        }],
    }],
    related_formats: &[],
};
