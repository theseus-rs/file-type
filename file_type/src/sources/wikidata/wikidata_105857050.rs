use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857050: FileFormat = FileFormat {
    id: 105_857_050,
    source_type: SourceType::Wikidata,
    name: "DISGCL script",
    extensions: &["gcl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x47, 0x43, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
