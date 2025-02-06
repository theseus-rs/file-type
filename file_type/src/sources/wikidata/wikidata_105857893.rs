use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857893: FileFormat = FileFormat {
    id: 105_857_893,
    source_type: SourceType::Wikidata,
    name: "Infinity Engine resource directory (v1)",
    extensions: &["key"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4B, 0x45, 0x59, 0x20, 0x56, 0x31, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
