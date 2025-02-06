use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29151555: FileFormat = FileFormat {
    id: 29_151_555,
    source_type: SourceType::Wikidata,
    name: "RAGE Package Format",
    extensions: &["rpf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x50, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
