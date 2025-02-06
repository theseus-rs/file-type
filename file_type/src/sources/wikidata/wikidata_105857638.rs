use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857638: FileFormat = FileFormat {
    id: 105_857_638,
    source_type: SourceType::Wikidata,
    name: "Art Icons Pro - IconProject",
    extensions: &["icpr"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB7, 0x49, 0x43, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
