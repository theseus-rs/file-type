use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855318: FileFormat = FileFormat {
    id: 105_855_318,
    source_type: SourceType::Wikidata,
    name: "The Need For Speed graphics",
    extensions: &["fsh"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x48, 0x50, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
