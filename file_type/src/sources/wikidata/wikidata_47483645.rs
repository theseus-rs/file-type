use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47483645: FileFormat = FileFormat {
    id: 47_483_645,
    puid: "wikidata/47483645",
    name: "Statistical Analysis System Data (Windows)",
    extensions: &["sas7bdat", "sd7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
