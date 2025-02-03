use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857544: FileFormat = FileFormat {
    id: 105_857_544,
    source_type: SourceType::Wikidata,
    name: "MagicDesk Icon",
    extensions: &["icn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1F, 0x1F])],
            },
        }],
    }],
    related_formats: &[],
};
