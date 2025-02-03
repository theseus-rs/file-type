use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853382: FileFormat = FileFormat {
    id: 105_853_382,
    source_type: SourceType::Wikidata,
    name: "Microsoft Speech Data",
    extensions: &["spd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x61, 0x74, 0x61, 0x00, 0x00, 0x01, 0x00, 0x33, 0x2D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
