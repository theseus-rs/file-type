use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856092: FileFormat = FileFormat {
    id: 105_856_092,
    source_type: SourceType::Wikidata,
    name: "Hyperspin DB format",
    extensions: &["dat"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x6D, 0x65, 0x6E, 0x75, 0x3E])],
            },
        }],
    }],
    related_formats: &[],
};
