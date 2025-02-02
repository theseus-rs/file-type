use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850117: FileFormat = FileFormat {
    id: 105_850_117,
    source_type: SourceType::Wikidata,
    name: "PopCom CP/M compressed executable",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2D, 0x70, 0x63, 0x31, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
