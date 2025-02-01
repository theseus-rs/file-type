use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131293815: FileFormat = FileFormat {
    id: 131_293_815,
    puid: "wikidata/131293815",
    name: "Tera Term macro source code file",
    extensions: &["ttl"],
    media_types: &["text/x-teratermmacro"],
    internal_signatures: &[],
    related_formats: &[],
};
