use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205523: FileFormat = FileFormat {
    id: 28_205_523,
    source_type: SourceType::Wikidata,
    name: "ICDRAW Group Icon File",
    extensions: &["ib3"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x43, 0x42, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
