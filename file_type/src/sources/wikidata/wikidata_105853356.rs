use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853356: FileFormat = FileFormat {
    id: 105_853_356,
    source_type: SourceType::Wikidata,
    name: "3D World Studio material",
    extensions: &["stf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x07, 0x19, 0x00, 0x00, 0x01, 0x09, 0x09,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
