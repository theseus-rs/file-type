use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850005: FileFormat = FileFormat {
    id: 105_850_005,
    source_type: SourceType::Wikidata,
    name: "Xilinx Core Generator System Project",
    extensions: &["cgp"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x45, 0x54, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
