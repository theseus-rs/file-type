use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111721131: FileFormat = FileFormat {
    id: 111_721_131,
    puid: "wikidata/111721131",
    name: "Fixed-format Fortran source",
    extensions: &["f", "f77", "for"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
