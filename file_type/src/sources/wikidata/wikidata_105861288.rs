use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861288: FileFormat = FileFormat {
    id: 105_861_288,
    source_type: SourceType::Wikidata,
    name: "Houdini LUT",
    extensions: &["lut"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x09, 0x09,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
