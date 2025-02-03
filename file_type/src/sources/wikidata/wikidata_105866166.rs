use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866166: FileFormat = FileFormat {
    id: 105_866_166,
    source_type: SourceType::Wikidata,
    name: "Windows 98 passwords",
    extensions: &["pwl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xE3, 0x82, 0x85, 0x96])],
            },
        }],
    }],
    related_formats: &[],
};
