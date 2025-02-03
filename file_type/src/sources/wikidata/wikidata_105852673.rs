use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852673: FileFormat = FileFormat {
    id: 105_852_673,
    source_type: SourceType::Wikidata,
    name: "Stata Data format (v114, LE)",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0x02, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
