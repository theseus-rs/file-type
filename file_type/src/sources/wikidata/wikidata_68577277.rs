use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_68577277: FileFormat = FileFormat {
    id: 68_577_277,
    source_type: SourceType::Wikidata,
    name: "Emacs/XEmacs byte-compiled Lisp file format",
    extensions: &["elc"],
    media_types: &["application/x-bytecode.elisp"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3B, 0x45, 0x4C, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
