use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111721061: FileFormat = FileFormat {
    id: 111_721_061,
    source_type: SourceType::Wikidata,
    name: "Free-format Fortran 90 source",
    extensions: &["f90"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
