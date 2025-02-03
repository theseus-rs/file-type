use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865076: FileFormat = FileFormat {
    id: 105_865_076,
    source_type: SourceType::Wikidata,
    name: "Kyoto Colorful Days game data archive",
    extensions: &["pac"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x42, 0x46, 0x42, 0x53, 0x49, 0x57, 0x59,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
