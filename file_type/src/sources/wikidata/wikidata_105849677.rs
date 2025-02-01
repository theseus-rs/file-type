use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849677: FileFormat = FileFormat {
    id: 105_849_677,
    puid: "wikidata/105849677",
    name: "CATIA Assembly (v5 r21)",
    extensions: &["catproduct"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x35, 0x5F, 0x43, 0x46, 0x56, 0x32, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
