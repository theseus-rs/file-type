use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111653322: FileFormat = FileFormat {
    id: 111_653_322,
    source_type: SourceType::Wikidata,
    name: "AARU Image Format",
    extensions: &["aaruf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x41, 0x52, 0x55, 0x46, 0x52, 0x4D, 0x54, 0x41,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
