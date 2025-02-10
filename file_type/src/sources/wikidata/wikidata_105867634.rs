use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867634: FileFormat = FileFormat {
    id: 105_867_634,
    source_type: SourceType::Wikidata,
    name: "Nastran input data",
    extensions: &["nas"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x24])],
            },
        }],
    }],
    related_formats: &[],
};
