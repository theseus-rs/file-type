use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861634: FileFormat = FileFormat {
    id: 105_861_634,
    puid: "wikidata/105861634",
    name: "Linux Software Map entry (v2)",
    extensions: &["lsm"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x65, 0x67, 0x69, 0x6E, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
