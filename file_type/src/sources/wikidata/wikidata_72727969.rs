use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72727969: FileFormat = FileFormat {
    id: 72_727_969,
    puid: "wikidata/72727969",
    name: "Windows for Pen Computing Notebook",
    extensions: &["ntb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xCA, 0xFE, 0x20, 0x00, 0x00, 0x02, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
