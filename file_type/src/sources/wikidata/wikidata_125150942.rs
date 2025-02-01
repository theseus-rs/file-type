use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125150942: FileFormat = FileFormat {
    id: 125_150_942,
    puid: "wikidata/125150942",
    name: "OmniGraffle Drawing (zipped)",
    extensions: &["graffle.zip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
