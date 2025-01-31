use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113375867: FileFormat = FileFormat {
    id: 113_375_867,
    puid: "wikidata/113375867",
    name: "Extended GEM Bit Image",
    extensions: &["ximg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
