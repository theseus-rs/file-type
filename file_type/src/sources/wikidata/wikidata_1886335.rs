use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1886335: FileFormat = FileFormat {
    id: 1_886_335,
    puid: "wikidata/1886335",
    name: "Maker Interchange Format",
    extensions: &["mif"],
    media_types: &["application/vnd.mif"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x4D, 0x49, 0x46, 0x46, 0x69, 0x6C, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
