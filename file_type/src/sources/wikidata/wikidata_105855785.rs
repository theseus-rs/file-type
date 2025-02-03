use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855785: FileFormat = FileFormat {
    id: 105_855_785,
    source_type: SourceType::Wikidata,
    name: "Mini Office text Document",
    extensions: &["doc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x0D, 0x80, 0xCF, 0x80, 0x81])],
            },
        }],
    }],
    related_formats: &[],
};
