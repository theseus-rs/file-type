use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858675: FileFormat = FileFormat {
    id: 105_858_675,
    source_type: SourceType::Wikidata,
    name: "J Wavelet Image Codec bitmap",
    extensions: &["wic"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFA, 0xDE, 0xBA, 0xBE, 0x01, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
