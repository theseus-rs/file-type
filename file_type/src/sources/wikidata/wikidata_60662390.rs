use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60662390: FileFormat = FileFormat {
    id: 60_662_390,
    puid: "wikidata/60662390",
    name: "Inkwriter Template",
    extensions: &["pdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
