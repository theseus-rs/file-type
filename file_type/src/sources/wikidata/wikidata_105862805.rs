use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862805: FileFormat = FileFormat {
    id: 105_862_805,
    source_type: SourceType::Wikidata,
    name: "OpenMPT Module",
    extensions: &["mptm"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x4D, 0x50, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
