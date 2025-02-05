use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849636: FileFormat = FileFormat {
    id: 105_849_636,
    source_type: SourceType::Wikidata,
    name: "GenCAD layout",
    extensions: &["cad"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x24, 0x48, 0x45, 0x41, 0x44, 0x45, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
