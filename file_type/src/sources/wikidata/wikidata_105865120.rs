use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865120: FileFormat = FileFormat {
    id: 105_865_120,
    source_type: SourceType::Wikidata,
    name: "Photomerge Composition",
    extensions: &["pmg"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x49, 0x53, 0x50, 0x49, 0x45, 0x43, 0x45, 0x53, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
