use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73516039: FileFormat = FileFormat {
    id: 73_516_039,
    puid: "wikidata/73516039",
    name: "Microsoft Private Key format",
    extensions: &["pkv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1E, 0xF1, 0xB5, 0xB0])],
            },
        }],
    }],
    related_formats: &[],
};
