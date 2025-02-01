use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853433: FileFormat = FileFormat {
    id: 105_853_433,
    puid: "wikidata/105853433",
    name: "Zeno format",
    extensions: &["zeno"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xA3, 0xA0, 0xD2, 0x55])],
            },
        }],
    }],
    related_formats: &[],
};
