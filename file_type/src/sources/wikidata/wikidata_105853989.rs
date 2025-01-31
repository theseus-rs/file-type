use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853989: FileFormat = FileFormat {
    id: 105_853_989,
    puid: "wikidata/105853989",
    name: "Squeeze It compressed archive",
    extensions: &["sqz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x4C, 0x53, 0x51, 0x5A, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
