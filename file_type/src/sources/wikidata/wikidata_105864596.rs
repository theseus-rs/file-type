use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864596: FileFormat = FileFormat {
    id: 105_864_596,
    source_type: SourceType::Wikidata,
    name: "PCE Sector Image disk image",
    extensions: &["psi"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x53, 0x49, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
