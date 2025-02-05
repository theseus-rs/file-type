use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854948: FileFormat = FileFormat {
    id: 105_854_948,
    source_type: SourceType::Wikidata,
    name: "GLB game data archive",
    extensions: &["glb"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x4C, 0x49, 0x42, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
