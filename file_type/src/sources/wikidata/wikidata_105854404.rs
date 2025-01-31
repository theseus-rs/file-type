use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854404: FileFormat = FileFormat {
    id: 105_854_404,
    puid: "wikidata/105854404",
    name: "SQWEZ compressed archive",
    extensions: &["sqz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x51, 0x57, 0x45, 0x5A, 0x20, 0x76])],
            },
        }],
    }],
    related_formats: &[],
};
