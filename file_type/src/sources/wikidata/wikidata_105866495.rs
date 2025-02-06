use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866495: FileFormat = FileFormat {
    id: 105_866_495,
    source_type: SourceType::Wikidata,
    name: "Pxlab experiment Design (with rem, var.1)",
    extensions: &["pxd"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2F, 0x2A])],
            },
        }],
    }],
    related_formats: &[],
};
