use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83884456: FileFormat = FileFormat {
    id: 83_884_456,
    source_type: SourceType::Wikidata,
    name: "Cardfile file format",
    extensions: &["crd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x47, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
