use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866558: FileFormat = FileFormat {
    id: 105_866_558,
    source_type: SourceType::Wikidata,
    name: "Papyrus Printer Information",
    extensions: &["pri"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x50, 0x52, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
