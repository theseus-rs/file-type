use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850039: FileFormat = FileFormat {
    id: 105_850_039,
    source_type: SourceType::Wikidata,
    name: "Colossal Raw asset Package",
    extensions: &["crp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x52, 0x41, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
