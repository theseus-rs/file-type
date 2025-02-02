use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111721061: FileFormat = FileFormat {
    id: 111_721_061,
    source_type: SourceType::Wikidata,
    name: "Free-format Fortran 90 source",
    extensions: &["f90"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
