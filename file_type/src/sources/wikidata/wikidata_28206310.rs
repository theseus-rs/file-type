use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206310: FileFormat = FileFormat {
    id: 28_206_310,
    puid: "wikidata/28206310",
    name: "Analyze HDR",
    extensions: &["hdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
