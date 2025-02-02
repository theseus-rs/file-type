use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857516: FileFormat = FileFormat {
    id: 105_857_516,
    source_type: SourceType::Wikidata,
    name: "iMON Setting file",
    extensions: &["imo"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3A, 0x5C])],
            },
        }],
    }],
    related_formats: &[],
};
