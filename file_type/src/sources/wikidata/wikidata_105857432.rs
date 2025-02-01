use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857432: FileFormat = FileFormat {
    id: 105_857_432,
    puid: "wikidata/105857432",
    name: "ParaJVE ROM",
    extensions: &["jverom"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x56, 0x45, 0x52, 0x4F, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
