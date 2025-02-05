use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850531: FileFormat = FileFormat {
    id: 105_850_531,
    source_type: SourceType::Wikidata,
    name: "Starcat disks catalogue",
    extensions: &["cat"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x74, 0x61, 0x72, 0x43, 0x61, 0x74, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
