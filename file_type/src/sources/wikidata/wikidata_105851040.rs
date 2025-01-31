use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851040: FileFormat = FileFormat {
    id: 105_851_040,
    puid: "wikidata/105851040",
    name: "RTFGEN Topic data",
    extensions: &["tpc"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5C, 0x64, 0x6F, 0x63, 0x73, 0x74, 0x61, 0x72, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
