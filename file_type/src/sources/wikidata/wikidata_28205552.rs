use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205552: FileFormat = FileFormat {
    id: 28_205_552,
    source_type: SourceType::Wikidata,
    name: "Nokia Group Graphic",
    extensions: &["ngg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x47, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
