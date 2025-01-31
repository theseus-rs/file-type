use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112661167: FileFormat = FileFormat {
    id: 112_661_167,
    puid: "wikidata/112661167",
    name: "Motion Analysis HTR File",
    extensions: &["htr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
