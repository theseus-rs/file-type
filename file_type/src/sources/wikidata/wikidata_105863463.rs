use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863463: FileFormat = FileFormat {
    id: 105_863_463,
    source_type: SourceType::Wikidata,
    name: "MINC1 Medical Imaging format",
    extensions: &["mnc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x44, 0x46, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
