use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863899: FileFormat = FileFormat {
    id: 105_863_899,
    puid: "wikidata/105863899",
    name: "Magnetic Game",
    extensions: &["mag"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x61, 0x53, 0x63])],
            },
        }],
    }],
    related_formats: &[],
};
