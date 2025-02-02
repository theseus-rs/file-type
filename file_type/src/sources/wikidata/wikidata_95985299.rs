use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_95985299: FileFormat = FileFormat {
    id: 95_985_299,
    source_type: SourceType::Wikidata,
    name: "Affymetrix CDF file format",
    extensions: &["cdf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x43, 0x44, 0x46, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};
