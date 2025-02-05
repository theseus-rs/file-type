use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863919: FileFormat = FileFormat {
    id: 105_863_919,
    source_type: SourceType::Wikidata,
    name: "Rising Eagle game data archive",
    extensions: &["mdat"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x41, 0x44, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
