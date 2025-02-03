use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862720: FileFormat = FileFormat {
    id: 105_862_720,
    source_type: SourceType::Wikidata,
    name: "iOS backup manifest index",
    extensions: &["mbdx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6D, 0x62, 0x64, 0x78, 0x02, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
