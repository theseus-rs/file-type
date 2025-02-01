use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67175428: FileFormat = FileFormat {
    id: 67_175_428,
    puid: "wikidata/67175428",
    name: "StarView Metafile",
    extensions: &["svm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x43, 0x4C, 0x4D, 0x54, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
