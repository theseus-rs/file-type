use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850545: FileFormat = FileFormat {
    id: 105_850_545,
    source_type: SourceType::Wikidata,
    name: "Javelin Country Driver",
    extensions: &["cdv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x61, 0x76, 0x65, 0x6C, 0x69, 0x6E, 0x43, 0x6F, 0x75, 0x6E, 0x74, 0x72,
                    0x79, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
