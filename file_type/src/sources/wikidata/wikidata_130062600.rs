use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130062600: FileFormat = FileFormat {
    id: 130_062_600,
    puid: "wikidata/130062600",
    name: "Kal source code file",
    extensions: &["kal", "kal"],
    media_types: &["application/kal", "text/kal"],
    internal_signatures: &[],
    related_formats: &[],
};
