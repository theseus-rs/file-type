use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_380319: FileFormat = FileFormat {
    id: 380_319,
    puid: "wikidata/380319",
    name: "dynamic-link library",
    extensions: &["dll", "dll"],
    media_types: &[
        "application/vnd.microsoft.portable-executable",
        "application/x-msdownload",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
