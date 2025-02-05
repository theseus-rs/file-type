use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861872: FileFormat = FileFormat {
    id: 105_861_872,
    source_type: SourceType::Wikidata,
    name: "Dega Movie capture",
    extensions: &["mmv"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4D, 0x56, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
