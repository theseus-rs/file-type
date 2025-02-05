use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865005: FileFormat = FileFormat {
    id: 105_865_005,
    source_type: SourceType::Wikidata,
    name: "PicoQuant Unified TTTR",
    extensions: &["ptu"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x51, 0x54, 0x54, 0x54, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
