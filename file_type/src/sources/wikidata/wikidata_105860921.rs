use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860921: FileFormat = FileFormat {
    id: 105_860_921,
    source_type: SourceType::Wikidata,
    name: "Wine Registry",
    extensions: &["reg"],
    media_types: &["application/x-wine-extension-reg"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x49, 0x4E, 0x45, 0x20, 0x52, 0x45, 0x47, 0x49, 0x53, 0x54, 0x52, 0x59,
                    0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
