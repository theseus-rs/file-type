use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857588: FileFormat = FileFormat {
    id: 105_857_588,
    source_type: SourceType::Wikidata,
    name: "ISAM table handler data",
    extensions: &["ism"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0xFE, 0x05, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
