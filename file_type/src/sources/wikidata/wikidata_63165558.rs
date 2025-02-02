use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63165558: FileFormat = FileFormat {
    id: 63_165_558,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office Binder Template for Windows, version 97-2003",
    extensions: &["obt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
