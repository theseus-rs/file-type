use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852568: FileFormat = FileFormat {
    id: 105_852_568,
    source_type: SourceType::Wikidata,
    name: "Diggles Saved Game File",
    extensions: &["sav"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x1F, 0x8B, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0B, 0xEC, 0xDD, 0x07,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
