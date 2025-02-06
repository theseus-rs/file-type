use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866628: FileFormat = FileFormat {
    id: 105_866_628,
    source_type: SourceType::Wikidata,
    name: "PlayStation high-speed 3D modeling data",
    extensions: &["pmd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x00, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
