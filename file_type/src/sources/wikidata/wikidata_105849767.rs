use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849767: FileFormat = FileFormat {
    id: 105_849_767,
    source_type: SourceType::Wikidata,
    name: "Cryptx encrypted data",
    extensions: &["crypt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x52, 0x59, 0x50, 0x54, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
