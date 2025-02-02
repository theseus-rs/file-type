use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856252: FileFormat = FileFormat {
    id: 105_856_252,
    source_type: SourceType::Wikidata,
    name: "DXM music",
    extensions: &["dxm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x43, 0x44, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
