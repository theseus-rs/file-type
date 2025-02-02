use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854818: FileFormat = FileFormat {
    id: 105_854_818,
    source_type: SourceType::Wikidata,
    name: "RAR compressed archive (v1.x)",
    extensions: &["rar"],
    media_types: &["application/vnd.rar"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x45, 0x7E, 0x5E])],
            },
        }],
    }],
    related_formats: &[],
};
