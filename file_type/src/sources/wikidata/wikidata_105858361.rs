use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858361: FileFormat = FileFormat {
    id: 105_858_361,
    source_type: SourceType::Wikidata,
    name: "EGrid32 Form",
    extensions: &["egr"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x47, 0x33, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
