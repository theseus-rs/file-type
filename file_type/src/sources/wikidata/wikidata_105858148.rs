use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858148: FileFormat = FileFormat {
    id: 105_858_148,
    source_type: SourceType::Wikidata,
    name: "Superbase printer driver",
    extensions: &["ini"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x6F, 0x6E, 0x74, 0x6E, 0x61, 0x6D, 0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
