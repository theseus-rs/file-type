use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131389470: FileFormat = FileFormat {
    id: 131_389_470,
    source_type: SourceType::Wikidata,
    name: "Varnish Configuration Language file format",
    extensions: &["vcl"],
    media_types: &["text/x-vclsrc"],
    signatures: &[],
    related_formats: &[],
};
