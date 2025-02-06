use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129086587: FileFormat = FileFormat {
    id: 129_086_587,
    source_type: SourceType::Wikidata,
    name: "Emacs Lisp file",
    extensions: &["el"],
    media_types: &["application/x-elisp", "text/x-elisp"],
    signatures: &[],
    related_formats: &[],
};
