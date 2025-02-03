use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850499: FileFormat = FileFormat {
    id: 105_850_499,
    source_type: SourceType::Wikidata,
    name: "Comic Book Creator document",
    extensions: &["cbcx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
