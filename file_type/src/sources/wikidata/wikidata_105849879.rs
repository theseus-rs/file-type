use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849879: FileFormat = FileFormat {
    id: 105_849_879,
    source_type: SourceType::Wikidata,
    name: "Visual Studio Civilization 5 Solution",
    extensions: &["civ5sln"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF])],
            },
        }],
    }],
    related_formats: &[],
};
