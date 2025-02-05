use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858726: FileFormat = FileFormat {
    id: 105_858_726,
    source_type: SourceType::Wikidata,
    name: "Borland Graphics Interface driver (v3.x)",
    extensions: &["bgi"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x42, 0x47, 0x44, 0x08, 0x08, 0x08, 0x08,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
