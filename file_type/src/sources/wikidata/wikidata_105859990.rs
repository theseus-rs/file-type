use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859990: FileFormat = FileFormat {
    id: 105_859_990,
    puid: "wikidata/105859990",
    name: "VSampler Sound Bank",
    extensions: &["vsb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x53, 0x42, 0x30, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
