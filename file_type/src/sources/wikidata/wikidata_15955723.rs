use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_15955723: FileFormat = FileFormat {
    id: 15_955_723,
    source_type: SourceType::Wikidata,
    name: "Python script",
    extensions: &["py"],
    media_types: &[
        "application/x-httpd-python",
        "text/x-python",
        "text/x-script.python",
    ],
    signatures: &[],
    related_formats: &[],
};
