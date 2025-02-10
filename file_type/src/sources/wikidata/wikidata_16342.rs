use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_16342: FileFormat = FileFormat {
    id: 16_342,
    source_type: SourceType::Wikidata,
    name: "XML Schema",
    extensions: &["xsd"],
    media_types: &["application/xml", "text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C])],
            },
        }],
    }],
    related_formats: &[],
};
