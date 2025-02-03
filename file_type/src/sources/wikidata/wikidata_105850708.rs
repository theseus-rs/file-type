use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850708: FileFormat = FileFormat {
    id: 105_850_708,
    source_type: SourceType::Wikidata,
    name: "ProHance Mouse Keys Definition table",
    extensions: &["kdh"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x9C])],
            },
        }],
    }],
    related_formats: &[],
};
