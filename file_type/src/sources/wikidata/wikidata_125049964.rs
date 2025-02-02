use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125049964: FileFormat = FileFormat {
    id: 125_049_964,
    source_type: SourceType::Wikidata,
    name: "Yoshimi Vector settings file",
    extensions: &["xvy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
