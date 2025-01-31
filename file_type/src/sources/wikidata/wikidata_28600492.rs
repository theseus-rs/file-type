use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600492: FileFormat = FileFormat {
    id: 28_600_492,
    puid: "wikidata/28600492",
    name: "Data Resource File",
    extensions: &["drs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
