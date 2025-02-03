use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859714: FileFormat = FileFormat {
    id: 105_859_714,
    source_type: SourceType::Wikidata,
    name: "VLBI Experiment",
    extensions: &["vex"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x45, 0x58, 0x5F, 0x72, 0x65, 0x76, 0x20, 0x3D, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
