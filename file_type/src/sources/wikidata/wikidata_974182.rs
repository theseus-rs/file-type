use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_974182: FileFormat = FileFormat {
    id: 974_182,
    source_type: SourceType::Wikidata,
    name: "Universal 3D",
    extensions: &["u3d"],
    media_types: &["model/u3d"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x33, 0x44, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
