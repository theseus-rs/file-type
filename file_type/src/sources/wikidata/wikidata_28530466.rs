use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28530466: FileFormat = FileFormat {
    id: 28_530_466,
    source_type: SourceType::Wikidata,
    name: "HyperChem Input File",
    extensions: &["hin"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x66, 0x6F, 0x72, 0x63, 0x65, 0x66, 0x69, 0x65, 0x6C, 0x64, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
