use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850435: FileFormat = FileFormat {
    id: 105_850_435,
    source_type: SourceType::Wikidata,
    name: "ColdFusion Component",
    extensions: &["cfc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x6F, 0x6D, 0x70, 0x6F, 0x6E, 0x65, 0x6E, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
