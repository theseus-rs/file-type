use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862183: FileFormat = FileFormat {
    id: 105_862_183,
    source_type: SourceType::Wikidata,
    name: "Misfit Model 3D model",
    extensions: &["mm3d"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x49, 0x53, 0x46, 0x49, 0x54, 0x33, 0x44,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
