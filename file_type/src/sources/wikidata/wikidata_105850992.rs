use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850992: FileFormat = FileFormat {
    id: 105_850_992,
    source_type: SourceType::Wikidata,
    name: "CodeWarrior Target Data (Little Endian)",
    extensions: &["tdt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6C, 0x6F, 0x6F, 0x63])],
            },
        }],
    }],
    related_formats: &[],
};
