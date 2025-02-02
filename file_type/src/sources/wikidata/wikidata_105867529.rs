use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867529: FileFormat = FileFormat {
    id: 105_867_529,
    source_type: SourceType::Wikidata,
    name: "Nuke script",
    extensions: &["nk"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x21, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
