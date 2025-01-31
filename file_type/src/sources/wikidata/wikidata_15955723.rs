use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_15955723: FileFormat = FileFormat {
    id: 15_955_723,
    puid: "wikidata/15955723",
    name: "Python script",
    extensions: &["py", "py", "py"],
    media_types: &[
        "application/x-httpd-python",
        "text/x-python",
        "text/x-script.python",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
