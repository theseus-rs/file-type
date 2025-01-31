use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127269093: FileFormat = FileFormat {
    id: 127_269_093,
    puid: "wikidata/127269093",
    name: "Bulk Data File",
    extensions: &["bdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
