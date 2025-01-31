use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100343334: FileFormat = FileFormat {
    id: 100_343_334,
    puid: "wikidata/100343334",
    name: "Corel Print House/Print Office Document, version 5",
    extensions: &["cpd", "cph", "cpo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
