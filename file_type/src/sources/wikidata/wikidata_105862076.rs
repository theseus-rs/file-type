use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862076: FileFormat = FileFormat {
    id: 105_862_076,
    source_type: SourceType::Wikidata,
    name: "mzTab format (with comment)",
    extensions: &["mztab"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4F, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
