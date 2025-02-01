use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853305: FileFormat = FileFormat {
    id: 105_853_305,
    puid: "wikidata/105853305",
    name: "Harvard Graphics presentation (v2.x)",
    extensions: &["shw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x48, 0x4F, 0x57])],
            },
        }],
    }],
    related_formats: &[],
};
