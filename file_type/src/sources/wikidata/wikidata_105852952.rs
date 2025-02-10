use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852952: FileFormat = FileFormat {
    id: 105_852_952,
    source_type: SourceType::Wikidata,
    name: "A'dam Music Composer Script (with rem)",
    extensions: &["scr"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23])],
            },
        }],
    }],
    related_formats: &[],
};
