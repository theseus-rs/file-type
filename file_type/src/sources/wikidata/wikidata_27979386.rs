use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979386: FileFormat = FileFormat {
    id: 27_979_386,
    puid: "wikidata/27979386",
    name: "Protected Interoperable File Format",
    extensions: &["piff"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x1C, 0x66, 0x74, 0x79, 0x70, 0x70, 0x69, 0x66, 0x66,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
