use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853836: FileFormat = FileFormat {
    id: 105_853_836,
    source_type: SourceType::Wikidata,
    name: "Dar archive",
    extensions: &["dar"],
    media_types: &["application/x-dar"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x00, 0x7B])],
            },
        }],
    }],
    related_formats: &[],
};
