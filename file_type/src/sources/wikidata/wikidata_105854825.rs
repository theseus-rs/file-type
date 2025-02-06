use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854825: FileFormat = FileFormat {
    id: 105_854_825,
    source_type: SourceType::Wikidata,
    name: "Audible Enhanced Audio",
    extensions: &["aax"],
    media_types: &["audio/vnd.audible.aax"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x24, 0x66, 0x74, 0x79, 0x70, 0x61, 0x61, 0x78, 0x20, 0x00,
                    0x00, 0x00, 0x01, 0x61, 0x61, 0x78, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
