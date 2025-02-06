use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855289: FileFormat = FileFormat {
    id: 105_855_289,
    source_type: SourceType::Wikidata,
    name: "Visual Studio F# Project",
    extensions: &["fsproj"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x3C])],
            },
        }],
    }],
    related_formats: &[],
};
