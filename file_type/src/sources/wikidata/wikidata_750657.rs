use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_750657: FileFormat = FileFormat {
    id: 750_657,
    source_type: SourceType::Wikidata,
    name: "Quicken Interchange Format",
    extensions: &["qif"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21, 0x54, 0x79, 0x70, 0x65, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
