use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126326662: FileFormat = FileFormat {
    id: 126_326_662,
    puid: "wikidata/126326662",
    name: "slrn configuration file",
    extensions: &["slrnrc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
