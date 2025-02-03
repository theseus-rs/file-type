use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856568: FileFormat = FileFormat {
    id: 105_856_568,
    source_type: SourceType::Wikidata,
    name: "Doom 2D Forever WAD archive",
    extensions: &["wad"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x46, 0x57, 0x41, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
