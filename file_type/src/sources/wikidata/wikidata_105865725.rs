use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865725: FileFormat = FileFormat {
    id: 105_865_725,
    source_type: SourceType::Wikidata,
    name: "KeyTronic Trakball profile",
    extensions: &["pro"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x41, 0x42, 0x45, 0x4C, 0x3A, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
