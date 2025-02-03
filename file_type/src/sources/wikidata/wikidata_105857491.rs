use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857491: FileFormat = FileFormat {
    id: 105_857_491,
    source_type: SourceType::Wikidata,
    name: "{smartassembly} project",
    extensions: &["{sa}proj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x53, 0x6D, 0x61, 0x72, 0x74, 0x41, 0x73, 0x73, 0x65, 0x6D, 0x62, 0x6C,
                    0x79, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x50, 0x72, 0x6F, 0x6A,
                    0x65, 0x63, 0x74, 0x49, 0x44, 0x3D, 0x22, 0x7B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
