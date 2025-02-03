use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862735: FileFormat = FileFormat {
    id: 105_862_735,
    source_type: SourceType::Wikidata,
    name: "SciADV MPK game data Package",
    extensions: &["mpk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x50, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
