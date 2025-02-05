use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859031: FileFormat = FileFormat {
    id: 105_859_031,
    source_type: SourceType::Wikidata,
    name: "GemCom Graphic bitmap",
    extensions: &["ggp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x47, 0x50, 0x46, 0x41, 0x49, 0x4B, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
