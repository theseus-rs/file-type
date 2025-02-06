use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860502: FileFormat = FileFormat {
    id: 105_860_502,
    source_type: SourceType::Wikidata,
    name: "Navitel 1.1 Map",
    extensions: &["rus"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x52, 0x55, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
