use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853155: FileFormat = FileFormat {
    id: 105_853_155,
    source_type: SourceType::Wikidata,
    name: "Microsoft Spider Solitaire Saved game",
    extensions: &["spidersolitairesave-ms"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x47, 0x4D, 0x48, 0x01, 0x00, 0x00, 0x00, 0x28, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
