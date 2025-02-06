use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849625: FileFormat = FileFormat {
    id: 105_849_625,
    source_type: SourceType::Wikidata,
    name: "Graphic Master charset",
    extensions: &["chr"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x50, 0x57, 0x63])],
            },
        }],
    }],
    related_formats: &[],
};
