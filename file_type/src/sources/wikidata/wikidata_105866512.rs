use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866512: FileFormat = FileFormat {
    id: 105_866_512,
    source_type: SourceType::Wikidata,
    name: "PhoneBook Backup",
    extensions: &["pbb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x45, 0x4C, 0x4C, 0x50, 0x42, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
