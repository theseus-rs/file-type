use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111721061: FileFormat = FileFormat {
    id: 111_721_061,
    puid: "wikidata/111721061",
    name: "Free-format Fortran 90 source",
    extensions: &["f90"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
