use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862902: FileFormat = FileFormat {
    id: 105_862_902,
    source_type: SourceType::Wikidata,
    name: "MedlySound module",
    extensions: &["mso"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x53, 0x4F, 0x42, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
