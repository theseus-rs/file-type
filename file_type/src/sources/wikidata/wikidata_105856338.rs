use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856338: FileFormat = FileFormat {
    id: 105_856_338,
    source_type: SourceType::Wikidata,
    name: "SoX Text Data audio",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3B, 0x20, 0x53, 0x61, 0x6D, 0x70, 0x6C, 0x65, 0x20, 0x52, 0x61, 0x74, 0x65,
                    0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
