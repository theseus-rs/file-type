use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850622: FileFormat = FileFormat {
    id: 105_850_622,
    puid: "wikidata/105850622",
    name: "Cookeo recipe",
    extensions: &["cok"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4F, 0x4F, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
