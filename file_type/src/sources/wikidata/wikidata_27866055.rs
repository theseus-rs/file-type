use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27866055: FileFormat = FileFormat {
    id: 27_866_055,
    puid: "wikidata/27866055",
    name: "bzip Archive",
    extensions: &["bz"],
    media_types: &["application/x-bzip"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x5A, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
