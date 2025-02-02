use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857693: FileFormat = FileFormat {
    id: 105_857_693,
    source_type: SourceType::Wikidata,
    name: "MAME Input (Extended Header)",
    extensions: &["inp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x49, 0x4E, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
