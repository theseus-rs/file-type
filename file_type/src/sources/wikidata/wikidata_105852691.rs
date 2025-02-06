use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852691: FileFormat = FileFormat {
    id: 105_852_691,
    source_type: SourceType::Wikidata,
    name: "STereoLithography (binary) (alt3) (gen)",
    extensions: &["stl"],
    media_types: &["model/x.stl-binary"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4F, 0x4C, 0x4F, 0x52, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
