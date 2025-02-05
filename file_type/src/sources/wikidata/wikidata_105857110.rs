use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857110: FileFormat = FileFormat {
    id: 105_857_110,
    source_type: SourceType::Wikidata,
    name: "gBurner Image",
    extensions: &["gbi"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x42, 0x49, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
