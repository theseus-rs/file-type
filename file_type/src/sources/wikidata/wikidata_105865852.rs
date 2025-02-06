use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865852: FileFormat = FileFormat {
    id: 105_865_852,
    source_type: SourceType::Wikidata,
    name: "Visual Studio PHP Project",
    extensions: &["phpproj"],
    media_types: &["text/xml"],
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
