use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857910: FileFormat = FileFormat {
    id: 105_857_910,
    source_type: SourceType::Wikidata,
    name: "IGOR Pro Text document",
    extensions: &["igtx", "itx", "text", "txt"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x47, 0x4F, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
