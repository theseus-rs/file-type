use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865968: FileFormat = FileFormat {
    id: 105_865_968,
    source_type: SourceType::Wikidata,
    name: "Paragon Backup Format image",
    extensions: &["pbf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x49, 0x6D, 0x67])],
            },
        }],
    }],
    related_formats: &[],
};
