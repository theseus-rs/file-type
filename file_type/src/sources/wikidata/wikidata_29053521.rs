use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29053521: FileFormat = FileFormat {
    id: 29_053_521,
    source_type: SourceType::Wikidata,
    name: "Kaitai Struct",
    extensions: &["ksy"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6D, 0x65, 0x74, 0x61, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
