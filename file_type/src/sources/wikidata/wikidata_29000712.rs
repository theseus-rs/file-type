use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000712: FileFormat = FileFormat {
    id: 29_000_712,
    source_type: SourceType::Wikidata,
    name: "TecPlot ASCII",
    extensions: &["tp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x49, 0x54, 0x4C, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
