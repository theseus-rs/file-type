use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205452: FileFormat = FileFormat {
    id: 28_205_452,
    source_type: SourceType::Wikidata,
    name: "J6I",
    extensions: &["j6i"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x80, 0x3E, 0x44, 0x53, 0x43, 0x49, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
