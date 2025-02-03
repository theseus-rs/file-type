use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967092: FileFormat = FileFormat {
    id: 27_967_092,
    source_type: SourceType::Wikidata,
    name: "Doom MUS",
    extensions: &["mus"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x55, 0x53, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
