use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866794: FileFormat = FileFormat {
    id: 105_866_794,
    source_type: SourceType::Wikidata,
    name: "Micrografx Designer Palette",
    extensions: &["pl4"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0x00, 0x01, 0x04, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
