use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866116: FileFormat = FileFormat {
    id: 105_866_116,
    source_type: SourceType::Wikidata,
    name: "KiCad Project (updated)",
    extensions: &["pro"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
