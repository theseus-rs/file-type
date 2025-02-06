use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865075: FileFormat = FileFormat {
    id: 105_865_075,
    source_type: SourceType::Wikidata,
    name: "Print Magic Page",
    extensions: &["pmp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4D, 0x50, 0x41, 0x47, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
