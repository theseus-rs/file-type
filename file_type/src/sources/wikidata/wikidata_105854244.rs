use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854244: FileFormat = FileFormat {
    id: 105_854_244,
    source_type: SourceType::Wikidata,
    name: "Xbox archive (generic)",
    extensions: &["xpr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x50, 0x52, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
