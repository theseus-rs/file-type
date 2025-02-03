use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866411: FileFormat = FileFormat {
    id: 105_866_411,
    source_type: SourceType::Wikidata,
    name: "MaxiPlan spreadsheet",
    extensions: &["plan"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4C, 0x42, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
