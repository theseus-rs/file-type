use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850981: FileFormat = FileFormat {
    id: 105_850_981,
    source_type: SourceType::Wikidata,
    name: "DB/TextWorks Database Menu Screen",
    extensions: &["tbm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x42, 0x4D, 0x20, 0x30, 0x30, 0x32, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
