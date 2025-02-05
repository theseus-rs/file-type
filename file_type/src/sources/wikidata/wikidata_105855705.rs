use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855705: FileFormat = FileFormat {
    id: 105_855_705,
    source_type: SourceType::Wikidata,
    name: "Origin Project",
    extensions: &["opj"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x50, 0x59, 0x41, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
