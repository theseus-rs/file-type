use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861291: FileFormat = FileFormat {
    id: 105_861_291,
    source_type: SourceType::Wikidata,
    name: "Tecplot Layout",
    extensions: &["lay"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x21, 0x4D, 0x43, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
