use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866937: FileFormat = FileFormat {
    id: 105_866_937,
    source_type: SourceType::Wikidata,
    name: "3DVIA Virtools behavioral object",
    extensions: &["nmo"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x65, 0x6D, 0x6F, 0x20, 0x46, 0x69])],
            },
        }],
    }],
    related_formats: &[],
};
