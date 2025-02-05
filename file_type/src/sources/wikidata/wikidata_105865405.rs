use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865405: FileFormat = FileFormat {
    id: 105_865_405,
    source_type: SourceType::Wikidata,
    name: "Parity Archive Volume Set (Par3)",
    extensions: &["pa3"],
    media_types: &["application/x-par3"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x41, 0x33, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
