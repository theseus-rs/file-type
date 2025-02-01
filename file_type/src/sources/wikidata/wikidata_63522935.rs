use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63522935: FileFormat = FileFormat {
    id: 63_522_935,
    puid: "wikidata/63522935",
    name: "Parametric Technology Pro/ENGINEER File Format",
    extensions: &["prt"],
    media_types: &["application/pro_eng"],
    internal_signatures: &[],
    related_formats: &[],
};
