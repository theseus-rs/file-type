use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66219660: FileFormat = FileFormat {
    id: 66_219_660,
    puid: "wikidata/66219660",
    name: "shtml",
    extensions: &["sht", "sht", "shtm", "shtm", "shtml", "shtml", "stm", "stm"],
    media_types: &[
        "text/x-server-parsed-html",
        "text/x-server-parsed-html",
        "text/x-server-parsed-html",
        "text/x-server-parsed-html",
        "text/x-server-parsed-html3",
        "text/x-server-parsed-html3",
        "text/x-server-parsed-html3",
        "text/x-server-parsed-html3",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
