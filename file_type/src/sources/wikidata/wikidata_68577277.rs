use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_68577277: FileType = FileType {
    file_format: &FileFormat {
        id: 68_577_277,
        source_type: SourceType::Wikidata,
        name: "Emacs/XEmacs byte-compiled Lisp file format",
        extensions: &["elc"],
        media_types: &["application/x-bytecode.elisp"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3B, 0x45, 0x4C, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
