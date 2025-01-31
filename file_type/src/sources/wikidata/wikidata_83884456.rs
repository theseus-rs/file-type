use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83884456: FileFormat = FileFormat {
    id: 83_884_456,
    puid: "wikidata/83884456",
    name: "Cardfile file format",
    extensions: &["crd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
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
