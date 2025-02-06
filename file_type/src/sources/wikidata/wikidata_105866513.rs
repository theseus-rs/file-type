use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866513: FileFormat = FileFormat {
    id: 105_866_513,
    source_type: SourceType::Wikidata,
    name: "BIS P3D ODOL model",
    extensions: &["p3d"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x44, 0x4F, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
