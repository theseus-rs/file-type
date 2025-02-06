use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857521: FileFormat = FileFormat {
    id: 105_857_521,
    source_type: SourceType::Wikidata,
    name: "MIRAX slide data (UTF-8)",
    extensions: &["ini"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x5B, 0x47, 0x45, 0x4E, 0x45, 0x52, 0x41, 0x4C, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
