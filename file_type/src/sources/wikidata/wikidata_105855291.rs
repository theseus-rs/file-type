use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855291: FileFormat = FileFormat {
    id: 105_855_291,
    source_type: SourceType::Wikidata,
    name: "HALion Sampler patch - bank",
    extensions: &["fxb"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x63, 0x6E, 0x4B, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
