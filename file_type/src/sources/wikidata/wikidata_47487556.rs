use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47487556: FileFormat = FileFormat {
    id: 47_487_556,
    puid: "wikidata/47487556",
    name: "TCR ebook",
    extensions: &["tcr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x21, 0x21, 0x38, 0x2D, 0x42, 0x69, 0x74, 0x21, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
