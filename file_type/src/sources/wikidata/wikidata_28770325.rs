use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28770325: FileFormat = FileFormat {
    id: 28_770_325,
    source_type: SourceType::Wikidata,
    name: "Lepton",
    extensions: &["lep"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xCF, 0x84])],
            },
        }],
    }],
    related_formats: &[],
};
