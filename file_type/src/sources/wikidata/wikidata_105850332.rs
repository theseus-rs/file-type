use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850332: FileFormat = FileFormat {
    id: 105_850_332,
    puid: "wikidata/105850332",
    name: "OCaml bytecode (native object)",
    extensions: &["cmx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x61, 0x6D, 0x6C, 0x31, 0x39, 0x39, 0x39, 0x59,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
