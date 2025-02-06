use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866841: FileFormat = FileFormat {
    id: 105_866_841,
    source_type: SourceType::Wikidata,
    name: "PV3D scene description data",
    extensions: &["pvd"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x56, 0x33, 0x44, 0x32, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
