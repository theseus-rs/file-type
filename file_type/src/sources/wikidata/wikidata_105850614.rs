use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850614: FileFormat = FileFormat {
    id: 105_850_614,
    source_type: SourceType::Wikidata,
    name: "Sundial Clearlook document",
    extensions: &["ctx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xCE, 0xCE, 0xF6, 0x00, 0x64, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
