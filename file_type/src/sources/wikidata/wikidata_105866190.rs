use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866190: FileFormat = FileFormat {
    id: 105_866_190,
    source_type: SourceType::Wikidata,
    name: "PhotoLine32 Document",
    extensions: &["pld"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x68, 0x6F, 0x74, 0x6F, 0x20, 0x4C, 0x69, 0x6E, 0x65, 0x20, 0x44, 0x6F,
                    0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
