use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865275: FileFormat = FileFormat {
    id: 105_865_275,
    source_type: SourceType::Wikidata,
    name: "Process Monitor Log (native format)",
    extensions: &["pml"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4D, 0x4C, 0x5F])],
            },
        }],
    }],
    related_formats: &[],
};
