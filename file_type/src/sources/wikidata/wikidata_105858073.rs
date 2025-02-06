use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858073: FileFormat = FileFormat {
    id: 105_858_073,
    source_type: SourceType::Wikidata,
    name: "Microbee tape image",
    extensions: &["tap"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x42, 0x45, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
