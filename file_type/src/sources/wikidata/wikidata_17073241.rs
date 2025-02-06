use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17073241: FileFormat = FileFormat {
    id: 17_073_241,
    source_type: SourceType::Wikidata,
    name: "Opera Show Format",
    extensions: &["html", "xhtml"],
    media_types: &["application/xhtml+xml", "text/html"],
    signatures: &[],
    related_formats: &[],
};
