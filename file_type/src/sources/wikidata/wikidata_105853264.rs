use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853264: FileFormat = FileFormat {
    id: 105_853_264,
    source_type: SourceType::Wikidata,
    name: "SIMPSON binary format",
    extensions: &["spe"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x49, 0x4D, 0x50, 0x0A, 0x4E, 0x50, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
