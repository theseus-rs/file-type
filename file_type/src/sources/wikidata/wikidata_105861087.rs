use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861087: FileFormat = FileFormat {
    id: 105_861_087,
    source_type: SourceType::Wikidata,
    name: "Quartus Library Mapping File",
    extensions: &["lmf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x49, 0x42, 0x52, 0x41, 0x52, 0x59, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
