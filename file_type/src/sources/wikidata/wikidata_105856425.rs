use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856425: FileFormat = FileFormat {
    id: 105_856_425,
    source_type: SourceType::Wikidata,
    name: "GFA Raytrace Animation (low-res)",
    extensions: &["wal"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x77, 0x61, 0x6C, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
