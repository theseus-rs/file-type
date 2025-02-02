use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859958: FileFormat = FileFormat {
    id: 105_859_958,
    source_type: SourceType::Wikidata,
    name: "ALRS H.264 DVR video",
    extensions: &["264"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4C, 0x52, 0x53, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
