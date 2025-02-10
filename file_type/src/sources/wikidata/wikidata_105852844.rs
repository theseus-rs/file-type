use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852844: FileFormat = FileFormat {
    id: 105_852_844,
    source_type: SourceType::Wikidata,
    name: "SuperTux World Map File (with rem)",
    extensions: &["stwm"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3B])],
            },
        }],
    }],
    related_formats: &[],
};
