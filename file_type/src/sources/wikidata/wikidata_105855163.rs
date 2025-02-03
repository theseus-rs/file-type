use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855163: FileFormat = FileFormat {
    id: 105_855_163,
    source_type: SourceType::Wikidata,
    name: "Amiga bitmap Font (var.2)",
    extensions: &["font"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0F, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
