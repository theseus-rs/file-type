use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855482: FileFormat = FileFormat {
    id: 105_855_482,
    source_type: SourceType::Wikidata,
    name: "Famtasia movie capture",
    extensions: &["fmv"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4D, 0x56, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
