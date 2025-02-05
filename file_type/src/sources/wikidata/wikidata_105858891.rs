use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858891: FileFormat = FileFormat {
    id: 105_858_891,
    source_type: SourceType::Wikidata,
    name: "Microsoft Store download package",
    extensions: &["box"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0x00, 0xC5, 0xB0])],
            },
        }],
    }],
    related_formats: &[],
};
