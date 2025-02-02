use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34284525: FileFormat = FileFormat {
    id: 34_284_525,
    source_type: SourceType::Wikidata,
    name: "Perl script",
    extensions: &["pl"],
    media_types: &[
        "application/x-httpd-perl",
        "application/x-perl",
        "text/x-perl",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
