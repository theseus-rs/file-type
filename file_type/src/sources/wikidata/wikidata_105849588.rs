use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849588: FileFormat = FileFormat {
    id: 105_849_588,
    source_type: SourceType::Wikidata,
    name: "Ventura Publisher Caption",
    extensions: &["cap"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40])],
            },
        }],
    }],
    related_formats: &[],
};
