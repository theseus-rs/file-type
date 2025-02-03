use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63106845: FileFormat = FileFormat {
    id: 63_106_845,
    source_type: SourceType::Wikidata,
    name: "Microsoft Office Binder Template for Windows",
    extensions: &["obt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
