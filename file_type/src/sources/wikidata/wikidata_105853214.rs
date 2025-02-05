use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853214: FileFormat = FileFormat {
    id: 105_853_214,
    source_type: SourceType::Wikidata,
    name: "GFA Raytrace compressed Animation (hi-res)",
    extensions: &["sah"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x61, 0x68, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
