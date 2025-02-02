use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63165182: FileFormat = FileFormat {
    id: 63_165_182,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office Binder Wizard for Windows",
    extensions: &["obz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
