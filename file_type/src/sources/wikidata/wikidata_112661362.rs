use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112661362: FileFormat = FileFormat {
    id: 112_661_362,
    puid: "wikidata/112661362",
    name: "Motion Analysis TRC File",
    extensions: &["trc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
