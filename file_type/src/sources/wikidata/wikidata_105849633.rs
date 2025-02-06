use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849633: FileFormat = FileFormat {
    id: 105_849_633,
    source_type: SourceType::Wikidata,
    name: "Aladdin 4D Spline",
    extensions: &["csp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x53, 0x50, 0x40])],
            },
        }],
    }],
    related_formats: &[],
};
