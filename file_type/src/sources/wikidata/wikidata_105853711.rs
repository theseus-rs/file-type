use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853711: FileFormat = FileFormat {
    id: 105_853_711,
    source_type: SourceType::Wikidata,
    name: "Animecha Animation Data",
    extensions: &["amd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4D, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
