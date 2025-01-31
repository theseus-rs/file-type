use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129086587: FileFormat = FileFormat {
    id: 129_086_587,
    puid: "wikidata/129086587",
    name: "Emacs Lisp file",
    extensions: &["el", "el"],
    media_types: &["application/x-elisp", "text/x-elisp"],
    internal_signatures: &[],
    related_formats: &[],
};
