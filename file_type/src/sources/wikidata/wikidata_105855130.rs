use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855130: FileFormat = FileFormat {
    id: 105_855_130,
    source_type: SourceType::Wikidata,
    name: "Dynamix Font data container",
    extensions: &["blk", "fnt"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4E, 0x54, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
