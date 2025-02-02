use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865611: FileFormat = FileFormat {
    id: 105_865_611,
    source_type: SourceType::Wikidata,
    name: "PVM Volume format",
    extensions: &["pvm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x56, 0x4D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
