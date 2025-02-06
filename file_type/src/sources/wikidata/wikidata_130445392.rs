use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130445392: FileFormat = FileFormat {
    id: 130_445_392,
    source_type: SourceType::Wikidata,
    name: "GnuCash XML Format",
    extensions: &["gnucash"],
    media_types: &["application/xml"],
    signatures: &[],
    related_formats: &[],
};
