use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852141: FileFormat = FileFormat {
    id: 105_852_141,
    source_type: SourceType::Wikidata,
    name: "Dynamix Screen data container",
    extensions: &["scr"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x43, 0x52, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
