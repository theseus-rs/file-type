use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121788783: FileFormat = FileFormat {
    id: 121_788_783,
    puid: "wikidata/121788783",
    name: "Yamaha PSR Disk Manager File",
    extensions: &["mng"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
