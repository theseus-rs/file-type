use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72825441: FileFormat = FileFormat {
    id: 72_825_441,
    puid: "wikidata/72825441",
    name: "Psion Organiser Block data",
    extensions: &["obx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x52, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
