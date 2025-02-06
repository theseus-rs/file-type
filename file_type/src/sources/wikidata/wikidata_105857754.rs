use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857754: FileFormat = FileFormat {
    id: 105_857_754,
    source_type: SourceType::Wikidata,
    name: "Infinity Engine archive (v1)",
    extensions: &["bif"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x49, 0x46, 0x46, 0x56, 0x31, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
