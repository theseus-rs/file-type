use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206125: FileFormat = FileFormat {
    id: 28_206_125,
    source_type: SourceType::Wikidata,
    name: "Flexible Line Interpretation",
    extensions: &["fli"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x3B])],
            },
        }],
    }],
    related_formats: &[],
};
