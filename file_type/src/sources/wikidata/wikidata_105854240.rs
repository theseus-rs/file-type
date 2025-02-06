use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854240: FileFormat = FileFormat {
    id: 105_854_240,
    source_type: SourceType::Wikidata,
    name: "Yamazaki Zipper compressed archive",
    extensions: &["yz1"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x79, 0x7A])],
            },
        }],
    }],
    related_formats: &[],
};
