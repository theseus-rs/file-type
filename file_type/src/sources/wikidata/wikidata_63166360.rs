use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63166360: FileFormat = FileFormat {
    id: 63_166_360,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office Binder Wizard for Windows, version 97-2003",
    extensions: &["obz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
