use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864386: FileFormat = FileFormat {
    id: 105_864_386,
    source_type: SourceType::Wikidata,
    name: "Pro Trekkr module",
    extensions: &["ptk"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x57, 0x4E, 0x4E, 0x53, 0x4E, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
