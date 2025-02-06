use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863688: FileFormat = FileFormat {
    id: 105_863_688,
    source_type: SourceType::Wikidata,
    name: "MoRay 3D Model",
    extensions: &["mdl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x44, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
