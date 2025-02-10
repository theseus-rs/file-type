use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29465145: FileFormat = FileFormat {
    id: 29_465_145,
    source_type: SourceType::Wikidata,
    name: "Valve Material Type",
    extensions: &["vmt"],
    media_types: &["application/vnd.valve.source.material"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x22])],
            },
        }],
    }],
    related_formats: &[],
};
