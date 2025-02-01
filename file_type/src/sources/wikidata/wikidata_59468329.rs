use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59468329: FileFormat = FileFormat {
    id: 59_468_329,
    puid: "wikidata/59468329",
    name: "Statistical Analysis System Data XPT (Unix)",
    extensions: &["xpt"],
    media_types: &["application/x-sas-xport"],
    internal_signatures: &[],
    related_formats: &[],
};
