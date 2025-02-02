use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1196805: FileFormat = FileFormat {
    id: 1_196_805,
    source_type: SourceType::Wikidata,
    name: "Resource Interchange File Format",
    extensions: &["riff"],
    media_types: &["application/x-riff"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x49, 0x46, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
