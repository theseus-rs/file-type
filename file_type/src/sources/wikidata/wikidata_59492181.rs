use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59492181: FileFormat = FileFormat {
    id: 59_492_181,
    puid: "wikidata/59492181",
    name: "Statistical Analysis System Catalog (Windows)",
    extensions: &["sas7bcat", "sc7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
