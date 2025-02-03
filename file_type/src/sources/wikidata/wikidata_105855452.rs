use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855452: FileFormat = FileFormat {
    id: 105_855_452,
    source_type: SourceType::Wikidata,
    name: "Grand Theft Auto Fake Text",
    extensions: &["fxt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xBF, 0x34, 0xEE, 0x82, 0x9F, 0x91, 0x1E])],
            },
        }],
    }],
    related_formats: &[],
};
