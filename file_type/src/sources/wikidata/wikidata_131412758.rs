use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131412758: FileFormat = FileFormat {
    id: 131_412_758,
    source_type: SourceType::Wikidata,
    name: "VimL script file",
    extensions: &["exrc", "gvimrc", "vim", "vimrc"],
    media_types: &["text/x-vim"],
    internal_signatures: &[],
    related_formats: &[],
};
