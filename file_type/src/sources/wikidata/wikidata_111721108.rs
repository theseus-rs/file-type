use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111721108: FileFormat = FileFormat {
    id: 111_721_108,
    source_type: SourceType::Wikidata,
    name: "Free-format Fortran 95 source",
    extensions: &["f95"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
