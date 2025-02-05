use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000727: FileFormat = FileFormat {
    id: 29_000_727,
    source_type: SourceType::Wikidata,
    name: "Digistar II VLA",
    extensions: &["vla"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x65, 0x74, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
