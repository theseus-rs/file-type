use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855710: FileFormat = FileFormat {
    id: 105_855_710,
    source_type: SourceType::Wikidata,
    name: "ObjectVision Datafile",
    extensions: &["ovd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x19, 0xA4, 0x14, 0x00, 0x24, 0x00, 0x02,
                    0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
