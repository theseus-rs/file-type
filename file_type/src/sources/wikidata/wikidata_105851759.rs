use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851759: FileFormat = FileFormat {
    id: 105_851_759,
    source_type: SourceType::Wikidata,
    name: "STereoLithography (binary) (alt2) (gen)",
    extensions: &["stl"],
    media_types: &["model/x.stl-binary"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x6F, 0x6C, 0x69, 0x64, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
