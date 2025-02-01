use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853949: FileFormat = FileFormat {
    id: 105_853_949,
    puid: "wikidata/105853949",
    name: "Dzip compressed archive (v2.x)",
    extensions: &["dz"],
    media_types: &["application/x-dzip"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x5A, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
