use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111721131: FileFormat = FileFormat {
    id: 111_721_131,
    source_type: SourceType::Wikidata,
    name: "Fixed-format Fortran source",
    extensions: &["f", "f77", "for"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
