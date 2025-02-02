use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858098: FileFormat = FileFormat {
    id: 105_858_098,
    source_type: SourceType::Wikidata,
    name: "HP Logical Interchange Format disk image",
    extensions: &["lif"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x80, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
