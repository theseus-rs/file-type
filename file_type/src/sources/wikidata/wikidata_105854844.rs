use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854844: FileFormat = FileFormat {
    id: 105_854_844,
    source_type: SourceType::Wikidata,
    name: "Quest Adventure Script (UTF-8) (v5)",
    extensions: &["aslx"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x3C])],
            },
        }],
    }],
    related_formats: &[],
};
