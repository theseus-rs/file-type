use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_43989854: FileFormat = FileFormat {
    id: 43_989_854,
    puid: "wikidata/43989854",
    name: "ATCO-CIF",
    extensions: &["cif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x54, 0x43, 0x4F, 0x2D, 0x43, 0x49, 0x46,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
