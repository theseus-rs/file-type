use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205363: FileFormat = FileFormat {
    id: 28_205_363,
    puid: "wikidata/28205363",
    name: "KDC (P-Series)",
    extensions: &["kdc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
