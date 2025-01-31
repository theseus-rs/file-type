use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852049: FileFormat = FileFormat {
    id: 105_852_049,
    puid: "wikidata/105852049",
    name: "SignPlot traffic sign data",
    extensions: &["sp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x49, 0x47, 0x4E, 0x50, 0x4C, 0x4F, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
