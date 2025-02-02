use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967402: FileFormat = FileFormat {
    id: 27_967_402,
    source_type: SourceType::Wikidata,
    name: "Beni Tracker module",
    extensions: &["pis"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06])],
            },
        }],
    }],
    related_formats: &[],
};
