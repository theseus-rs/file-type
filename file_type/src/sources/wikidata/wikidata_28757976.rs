use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757976: FileFormat = FileFormat {
    id: 28_757_976,
    source_type: SourceType::Wikidata,
    name: "IMP",
    extensions: &["imp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x4D, 0x50, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
