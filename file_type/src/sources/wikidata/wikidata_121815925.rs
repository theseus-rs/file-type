use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121815925: FileFormat = FileFormat {
    id: 121_815_925,
    puid: "wikidata/121815925",
    name: "GST Art File 2",
    extensions: &["art"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
