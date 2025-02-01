use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27487343: FileFormat = FileFormat {
    id: 27_487_343,
    puid: "wikidata/27487343",
    name: "Shapefile spatial index part 1",
    extensions: &["sbn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
