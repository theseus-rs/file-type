use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851984: FileFormat = FileFormat {
    id: 105_851_984,
    source_type: SourceType::Wikidata,
    name: "Sweet Home 3D design (generic)",
    extensions: &["sh3d"],
    media_types: &["application/zip"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};
