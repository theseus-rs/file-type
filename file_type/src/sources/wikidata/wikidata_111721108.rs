use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111721108: FileFormat = FileFormat {
    id: 111_721_108,
    puid: "wikidata/111721108",
    name: "Free-format Fortran 95 source",
    extensions: &["f95"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
