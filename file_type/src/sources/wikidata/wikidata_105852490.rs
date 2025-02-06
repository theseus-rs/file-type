use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852490: FileFormat = FileFormat {
    id: 105_852_490,
    source_type: SourceType::Wikidata,
    name: "saltpack detached signature (ASCII)",
    extensions: &["asc"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x45, 0x47, 0x49, 0x4E, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
