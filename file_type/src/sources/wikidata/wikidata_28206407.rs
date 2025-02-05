use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206407: FileFormat = FileFormat {
    id: 28_206_407,
    source_type: SourceType::Wikidata,
    name: "Jeff's Image Format",
    extensions: &["jif"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x49, 0x46, 0x39, 0x39, 0x61])],
            },
        }],
    }],
    related_formats: &[],
};
