use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849739: FileFormat = FileFormat {
    id: 105_849_739,
    source_type: SourceType::Wikidata,
    name: "Generic network capture file (XCP style)",
    extensions: &["cap"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x43, 0x50, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
