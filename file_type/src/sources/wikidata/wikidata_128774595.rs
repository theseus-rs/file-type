use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128774595: FileFormat = FileFormat {
    id: 128_774_595,
    source_type: SourceType::Wikidata,
    name: "Common Lisp file format",
    extensions: &["cl"],
    media_types: &["text/x-common-lisp"],
    internal_signatures: &[],
    related_formats: &[],
};
