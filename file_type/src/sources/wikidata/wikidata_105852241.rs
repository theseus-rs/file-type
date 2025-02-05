use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852241: FileFormat = FileFormat {
    id: 105_852_241,
    source_type: SourceType::Wikidata,
    name: "Birth of The Empire / Federation savegame",
    extensions: &["sav"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x6F, 0x74, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
