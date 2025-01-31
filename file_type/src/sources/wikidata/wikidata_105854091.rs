use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854091: FileFormat = FileFormat {
    id: 105_854_091,
    puid: "wikidata/105854091",
    name: "CSArc compressed archive",
    extensions: &["csa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x53, 0x41, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
