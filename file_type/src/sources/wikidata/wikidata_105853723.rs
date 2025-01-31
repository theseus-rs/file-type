use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853723: FileFormat = FileFormat {
    id: 105_853_723,
    puid: "wikidata/105853723",
    name: "Chasm BIN archive",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x53, 0x69, 0x64])],
            },
        }],
    }],
    related_formats: &[],
};
