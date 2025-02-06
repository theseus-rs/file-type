use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854520: FileFormat = FileFormat {
    id: 105_854_520,
    source_type: SourceType::Wikidata,
    name: "CPIO archive (byte swapped binary)",
    extensions: &["cpio"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x71, 0xC7])],
            },
        }],
    }],
    related_formats: &[],
};
