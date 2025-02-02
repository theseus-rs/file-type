use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861713: FileFormat = FileFormat {
    id: 105_861_713,
    source_type: SourceType::Wikidata,
    name: "MegaZeux MZM2 image",
    extensions: &["mzm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x5A, 0x4D, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
