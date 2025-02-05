use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855066: FileFormat = FileFormat {
    id: 105_855_066,
    source_type: SourceType::Wikidata,
    name: "RNC / PRO-PACK archive",
    extensions: &["rnc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x4E, 0x43, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
