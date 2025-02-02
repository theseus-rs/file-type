use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863393: FileFormat = FileFormat {
    id: 105_863_393,
    source_type: SourceType::Wikidata,
    name: "Meyer/Glass Interactive game data Format",
    extensions: &["mgf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x47, 0x46, 0x73])],
            },
        }],
    }],
    related_formats: &[],
};
