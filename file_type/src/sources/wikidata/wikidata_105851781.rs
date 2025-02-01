use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851781: FileFormat = FileFormat {
    id: 105_851_781,
    puid: "wikidata/105851781",
    name: "Solitaire Setty Project",
    extensions: &["ssp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x4F, 0x4C, 0x53, 0x45, 0x54, 0x54, 0x59,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
