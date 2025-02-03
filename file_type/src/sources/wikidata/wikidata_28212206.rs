use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28212206: FileFormat = FileFormat {
    id: 28_212_206,
    source_type: SourceType::Wikidata,
    name: "Object File Format, text variant",
    extensions: &["aoff"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0x61, 0x6D, 0x65, 0x09, 0x09])],
            },
        }],
    }],
    related_formats: &[],
};
