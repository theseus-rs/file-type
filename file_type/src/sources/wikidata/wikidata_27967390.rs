use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967390: FileFormat = FileFormat {
    id: 27_967_390,
    source_type: SourceType::Wikidata,
    name: "Adlib Tracker II instrument",
    extensions: &["a2i"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5F, 0x61, 0x32, 0x69, 0x6E, 0x73, 0x5F])],
            },
        }],
    }],
    related_formats: &[],
};
