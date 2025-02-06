use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866781: FileFormat = FileFormat {
    id: 105_866_781,
    source_type: SourceType::Wikidata,
    name: "Palm TealInfo",
    extensions: &["pdb"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x49, 0x6E, 0x66, 0x6F, 0x54, 0x6C, 0x49, 0x66,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
