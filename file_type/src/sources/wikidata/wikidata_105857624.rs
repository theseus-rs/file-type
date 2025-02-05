use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857624: FileFormat = FileFormat {
    id: 105_857_624,
    source_type: SourceType::Wikidata,
    name: "RC750 PICCOLINE disk image",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x43, 0x37, 0x35, 0x30, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
