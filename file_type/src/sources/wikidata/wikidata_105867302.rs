use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867302: FileFormat = FileFormat {
    id: 105_867_302,
    source_type: SourceType::Wikidata,
    name: "Channel Data File",
    extensions: &["ndf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x44, 0x46, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
