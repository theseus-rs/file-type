use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865409: FileFormat = FileFormat {
    id: 105_865_409,
    source_type: SourceType::Wikidata,
    name: "Palm FireViewer bitmap",
    extensions: &["pdb"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x76, 0x49, 0x4D, 0x47, 0x56, 0x69, 0x65, 0x77,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
