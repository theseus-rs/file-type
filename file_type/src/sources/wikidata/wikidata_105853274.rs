use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853274: FileFormat = FileFormat {
    id: 105_853_274,
    source_type: SourceType::Wikidata,
    name: "SynWrite Snippet",
    extensions: &["synw-snippet"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0x61, 0x6D, 0x65, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
