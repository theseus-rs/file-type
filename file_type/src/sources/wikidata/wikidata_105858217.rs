use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858217: FileFormat = FileFormat {
    id: 105_858_217,
    puid: "wikidata/105858217",
    name: "Encapsulated PostScript Interchange",
    extensions: &["epsi"],
    media_types: &["application/postscript"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
