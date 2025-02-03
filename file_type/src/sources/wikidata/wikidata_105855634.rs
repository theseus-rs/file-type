use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855634: FileFormat = FileFormat {
    id: 105_855_634,
    source_type: SourceType::Wikidata,
    name: "Olympus digital camera RAW image (IIRS)",
    extensions: &["orf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x49, 0x52, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
