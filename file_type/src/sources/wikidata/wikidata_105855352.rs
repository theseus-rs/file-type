use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855352: FileFormat = FileFormat {
    id: 105_855_352,
    source_type: SourceType::Wikidata,
    name: "VFONT Font",
    extensions: &["fnt"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x56, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
