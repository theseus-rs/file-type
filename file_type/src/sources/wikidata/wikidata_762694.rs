use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_762694: FileFormat = FileFormat {
    id: 762_694,
    puid: "wikidata/762694",
    name: "PostScript Printer Description",
    extensions: &["ppd"],
    media_types: &["application/vnd.cups-ppd"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x50, 0x50, 0x44, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
