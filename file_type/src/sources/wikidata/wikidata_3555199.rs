use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3555199: FileFormat = FileFormat {
    id: 3_555_199,
    source_type: SourceType::Wikidata,
    name: "VEG",
    extensions: &["veg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0x69, 0x66, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
