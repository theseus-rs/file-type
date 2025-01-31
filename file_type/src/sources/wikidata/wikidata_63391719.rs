use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63391719: FileFormat = FileFormat {
    id: 63_391_719,
    puid: "wikidata/63391719",
    name: "Microsoft Access Database, version 2007",
    extensions: &["accdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x01, 0x00, 0x00, 0x53, 0x74, 0x61, 0x6E, 0x64, 0x61, 0x72, 0x64, 0x20,
                    0x41, 0x43, 0x45, 0x20, 0x44, 0x42,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
