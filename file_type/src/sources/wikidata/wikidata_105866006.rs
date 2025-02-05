use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866006: FileFormat = FileFormat {
    id: 105_866_006,
    source_type: SourceType::Wikidata,
    name: "Crouzet Logic Software M3 project",
    extensions: &["pm3"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x52, 0x4F, 0x55, 0x5A, 0x45, 0x54, 0x5F, 0x4D, 0x33,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
