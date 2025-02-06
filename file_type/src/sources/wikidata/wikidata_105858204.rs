use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858204: FileFormat = FileFormat {
    id: 105_858_204,
    source_type: SourceType::Wikidata,
    name: "EGrid32 Form properties template",
    extensions: &["egp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x47, 0x50, 0x52, 0x4F, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
