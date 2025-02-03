use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859694: FileFormat = FileFormat {
    id: 105_859_694,
    source_type: SourceType::Wikidata,
    name: "Xilinx instantiation template",
    extensions: &["vho"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2D, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
