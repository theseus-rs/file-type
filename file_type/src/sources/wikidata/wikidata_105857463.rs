use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857463: FileFormat = FileFormat {
    id: 105_857_463,
    source_type: SourceType::Wikidata,
    name: "Cyber Studio CAD-3D v2 object",
    extensions: &["3d2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3D, 0x02, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
