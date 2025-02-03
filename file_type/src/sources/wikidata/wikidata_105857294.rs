use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857294: FileFormat = FileFormat {
    id: 105_857_294,
    source_type: SourceType::Wikidata,
    name: "HEAD AFNI medical metadata",
    extensions: &["head"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0A, 0x74, 0x79, 0x70, 0x65, 0x20, 0x3D, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6E,
                    0x67, 0x2D, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x0A, 0x6E,
                    0x61, 0x6D, 0x65, 0x20, 0x3D, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
