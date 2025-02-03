use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859410: FileFormat = FileFormat {
    id: 105_859_410,
    source_type: SourceType::Wikidata,
    name: "Compaq QRST disk image",
    extensions: &["_01"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x51, 0x52, 0x53, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
