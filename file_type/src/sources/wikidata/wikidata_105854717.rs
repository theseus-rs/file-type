use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854717: FileFormat = FileFormat {
    id: 105_854_717,
    source_type: SourceType::Wikidata,
    name: "ZPack Lite compressed archive",
    extensions: &["zpk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x5A, 0x50, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
