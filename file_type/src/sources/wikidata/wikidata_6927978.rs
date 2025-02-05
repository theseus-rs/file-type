use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_6927978: FileFormat = FileFormat {
    id: 6_927_978,
    source_type: SourceType::Wikidata,
    name: "Mozilla Archive Format",
    extensions: &["maff"],
    media_types: &["application/x-maff"],
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
