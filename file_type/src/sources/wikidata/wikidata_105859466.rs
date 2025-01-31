use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859466: FileFormat = FileFormat {
    id: 105_859_466,
    puid: "wikidata/105859466",
    name: "QuickCompression compressed data (BE)",
    extensions: &["qcmp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x51, 0x43, 0x4D, 0x50, 0x00, 0x01, 0x00, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
