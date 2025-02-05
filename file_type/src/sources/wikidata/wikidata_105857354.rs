use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857354: FileFormat = FileFormat {
    id: 105_857_354,
    source_type: SourceType::Wikidata,
    name: "EditPad Pro Custom Syntax Coloring Scheme (ASCII)",
    extensions: &["jgcscs"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x47, 0x43, 0x53, 0x43, 0x53, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
