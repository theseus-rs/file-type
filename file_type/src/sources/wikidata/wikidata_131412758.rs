use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131412758: FileFormat = FileFormat {
    id: 131_412_758,
    puid: "wikidata/131412758",
    name: "VimL script file",
    extensions: &["exrc", "gvimrc", "vim", "vimrc"],
    media_types: &["text/x-vim", "text/x-vim", "text/x-vim", "text/x-vim"],
    internal_signatures: &[],
    related_formats: &[],
};
