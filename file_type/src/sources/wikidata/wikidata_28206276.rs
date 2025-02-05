use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206276: FileFormat = FileFormat {
    id: 28_206_276,
    source_type: SourceType::Wikidata,
    name: "IBM KIPS bitmap",
    extensions: &["kps"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x46, 0x49, 0x4D, 0x41, 0x47, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
