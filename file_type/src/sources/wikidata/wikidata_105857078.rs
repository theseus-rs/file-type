use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857078: FileFormat = FileFormat {
    id: 105_857_078,
    source_type: SourceType::Wikidata,
    name: "Windows Program Manager Group",
    extensions: &["grp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4D, 0x43, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
