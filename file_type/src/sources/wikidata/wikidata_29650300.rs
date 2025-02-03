use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650300: FileFormat = FileFormat {
    id: 29_650_300,
    source_type: SourceType::Wikidata,
    name: "PVK",
    extensions: &["pvk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB0, 0xB5, 0xF1, 0x1E])],
            },
        }],
    }],
    related_formats: &[],
};
