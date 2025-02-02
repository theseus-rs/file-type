use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851693: FileFormat = FileFormat {
    id: 105_851_693,
    source_type: SourceType::Wikidata,
    name: "Strong Name Key",
    extensions: &["snk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x07, 0x02, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x52, 0x53, 0x41, 0x32, 0x00,
                    0x04, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
