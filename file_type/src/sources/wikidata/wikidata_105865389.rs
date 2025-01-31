use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865389: FileFormat = FileFormat {
    id: 105_865_389,
    puid: "wikidata/105865389",
    name: "Nota Bene Printer definition (MS-DOS)",
    extensions: &["prn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3B, 0x50, 0x52, 0x3B, 0xAE, 0x53, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
