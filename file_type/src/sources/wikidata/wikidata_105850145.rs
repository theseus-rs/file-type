use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850145: FileFormat = FileFormat {
    id: 105_850_145,
    source_type: SourceType::Wikidata,
    name: "Black and White 2 game data script",
    extensions: &["chl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x48, 0x56, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
