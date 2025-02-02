use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859648: FileFormat = FileFormat {
    id: 105_859_648,
    source_type: SourceType::Wikidata,
    name: "Hamamatsu Virtual Microscope Specimen",
    extensions: &["vms"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x56, 0x69, 0x72, 0x74, 0x75, 0x61, 0x6C, 0x20, 0x4D, 0x69, 0x63, 0x72,
                    0x6F, 0x73, 0x63, 0x6F, 0x70, 0x65, 0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x6D,
                    0x65, 0x6E, 0x5D, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
