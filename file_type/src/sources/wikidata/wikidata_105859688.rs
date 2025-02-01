use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859688: FileFormat = FileFormat {
    id: 105_859_688,
    puid: "wikidata/105859688",
    name: "VSIX Manifest (2011) (UTF-8)",
    extensions: &["vsixmanifest"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x3C])],
            },
        }],
    }],
    related_formats: &[],
};
