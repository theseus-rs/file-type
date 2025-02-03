use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

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
    internal_signatures: &[],
    related_formats: &[],
};
