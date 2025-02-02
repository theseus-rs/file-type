use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867617: FileFormat = FileFormat {
    id: 105_867_617,
    source_type: SourceType::Wikidata,
    name: "Expert Help hypertext",
    extensions: &["ng"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x48, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
