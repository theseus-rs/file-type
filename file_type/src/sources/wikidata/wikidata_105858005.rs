use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858005: FileFormat = FileFormat {
    id: 105_858_005,
    source_type: SourceType::Wikidata,
    name: "Indigo Image",
    extensions: &["igi"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7D, 0x70, 0xF8, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
