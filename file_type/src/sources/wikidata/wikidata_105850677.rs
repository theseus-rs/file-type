use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850677: FileFormat = FileFormat {
    id: 105_850_677,
    source_type: SourceType::Wikidata,
    name: "Quick Pascal Keyboard map",
    extensions: &["key"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x51, 0x50, 0x00, 0x01, 0x38, 0x3F])],
            },
        }],
    }],
    related_formats: &[],
};
