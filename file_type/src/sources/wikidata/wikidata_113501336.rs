use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113501336: FileFormat = FileFormat {
    id: 113_501_336,
    puid: "wikidata/113501336",
    name: "PageMaker Mac Document 6.5-7.0",
    extensions: &["p65", "pmd", "pmt", "t65"],
    media_types: &[
        "application/vnd.pagemaker",
        "application/vnd.pagemaker",
        "application/vnd.pagemaker",
        "application/vnd.pagemaker",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
