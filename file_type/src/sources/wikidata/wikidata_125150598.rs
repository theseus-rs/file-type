use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125150598: FileFormat = FileFormat {
    id: 125_150_598,
    puid: "wikidata/125150598",
    name: "Gliffy diagram (gxml)",
    extensions: &["gxml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
